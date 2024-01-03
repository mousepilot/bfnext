use crate::db::{Db, InstancedPlayer, Player};
use anyhow::Result;
use chrono::prelude::*;
use dcso3::{
    coalition::Side, land::Land, net::Ucid, radians_to_degrees, LuaVec3, MizLua, Position3,
    Vector2, Vector3,
};
use fxhash::FxHashMap;
use smallvec::{smallvec, SmallVec};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct GibBraa {
    pub bearing: u16,
    pub range: u32,
    pub altitude: u32,
    pub heading: u16,
    pub speed: u16,
    pub age: u16,
    pub units: EwrUnits,
}

impl fmt::Display for GibBraa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (range_u, altitude_u, speed_u) = match self.units {
            EwrUnits::Imperial => ("nm", "ft", "kts"),
            EwrUnits::Metric => ("km", "m", "km/h"),
        };
        write!(
            f,
            "{:03} {:03}{} {:>5}{} {:03} {:04}{} {:03}s",
            self.bearing,
            self.range,
            range_u,
            self.altitude,
            altitude_u,
            self.heading,
            self.speed,
            speed_u,
            self.age
        )
    }
}

impl GibBraa {
    fn convert(&mut self, unit: EwrUnits) {
        match unit {
            EwrUnits::Metric => (),
            EwrUnits::Imperial => {
                self.range = self.range / 1852;
                self.altitude = (self.altitude as f64 * 1000. * 3.38084) as u32;
            }
        }
        self.units = unit;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Track {
    pos: Position3,
    velocity: Vector3,
    last: DateTime<Utc>,
    side: Side,
}

#[derive(Debug, Clone, Copy)]
pub enum EwrUnits {
    Imperial,
    Metric,
}

impl Default for EwrUnits {
    fn default() -> Self {
        Self::Metric
    }
}

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    enabled: bool,
    units: EwrUnits,
    last: DateTime<Utc>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            enabled: true,
            units: EwrUnits::default(),
            last: DateTime::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ewr {
    tracks: FxHashMap<Side, FxHashMap<Ucid, Track>>,
    player_state: FxHashMap<Ucid, PlayerState>,
}

impl Ewr {
    pub fn update_tracks(&mut self, lua: MizLua, db: &Db, now: DateTime<Utc>) -> Result<()> {
        let land = Land::singleton(lua)?;
        let players: SmallVec<[_; 64]> = db
            .instanced_players()
            .filter(|(_, _, inst)| inst.in_air)
            .collect();
        for (ewr_pos, side, ewr) in db.ewrs() {
            let range = (ewr.range as f64).powi(2);
            let tracks = self.tracks.entry(side).or_default();
            for (ucid, player, inst) in &players {
                let track = tracks.entry((*ucid).clone()).or_default();
                if track.last != now {
                    let dist = na::distance_squared(&ewr_pos.into(), &inst.position.p.0.into());
                    if dist <= range && land.is_visible(LuaVec3(ewr_pos), inst.position.p)? {
                        track.pos = inst.position;
                        track.velocity = inst.velocity;
                        track.last = now;
                        track.side = player.side;
                    }
                    dbg!(&track);
                }
            }
        }
        Ok(())
    }

    pub fn toggle(&mut self, ucid: &Ucid) -> bool {
        let st = self.player_state.entry(ucid.clone()).or_default();
        st.enabled = !st.enabled;
        st.enabled
    }

    pub fn set_units(&mut self, ucid: &Ucid, units: EwrUnits) {
        self.player_state.entry(ucid.clone()).or_default().units = units;
    }

    pub fn where_chicken(
        &mut self,
        now: DateTime<Utc>,
        friendly: bool,
        ucid: &Ucid,
        player: &Player,
        inst: &InstancedPlayer,
    ) -> SmallVec<[GibBraa; 64]> {
        let side = player.side;
        let pos = Vector2::new(inst.position.p.x, inst.position.p.z);
        let mut reports: SmallVec<[GibBraa; 64]> = smallvec![];
        let tracks = match self.tracks.get(&side) {
            Some(t) => t,
            None => return smallvec![],
        };
        let state = self.player_state.entry(ucid.clone()).or_default();
        if !state.enabled {
            return reports;
        }
        for track in tracks.values() {
            let age = (now - track.last).num_seconds();
            let include = (friendly && track.side == side) || (!friendly && track.side != side);
            if include && age <= 120 {
                let cpos = Vector2::new(track.pos.p.x, track.pos.p.z);
                let range = na::distance(&pos.into(), &cpos.into());
                let v = pos - cpos;
                let bearing = radians_to_degrees(v.y.atan2(v.x).abs());
                let heading = radians_to_degrees(cpos.y.atan2(cpos.x).abs());
                let speed = track.velocity.magnitude();
                let altitude = track.pos.p.y / 1000.;
                reports.push(GibBraa {
                    range: range as u32,
                    heading: heading as u16,
                    altitude: altitude as u32,
                    bearing: bearing as u16,
                    age: age as u16,
                    speed: speed as u16,
                    units: EwrUnits::Metric,
                })
            }
        }
        if reports.is_empty() {
            return reports;
        }
        reports.sort_by_key(|r| r.range);
        while reports.len() > 10 {
            reports.pop();
        }
        let since_last = (now - state.last).num_seconds();
        if since_last >= 60
            || reports[0].range <= 20000
            || (reports[0].range <= 40000 && since_last >= 30)
        {
            state.last = now;
            reports.iter_mut().for_each(|r| r.convert(state.units));
            reports
        } else {
            smallvec![]
        }
    }
}
