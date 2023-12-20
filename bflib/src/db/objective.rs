use super::{Db, GroupId, ObjGroupClass, Objective, ObjectiveId};
use crate::{
    group, objective, objective_mut,
    spawnctx::{Despawn, SpawnCtx},
    unit,
};
use anyhow::{anyhow, Result};
use chrono::{prelude::*, Duration};
use dcso3::{coalition::Side, env::miz::MizIndex, MizLua, Vector2};
use fxhash::FxHashMap;
use log::{debug, error};
use smallvec::{smallvec, SmallVec};

impl Db {
    fn compute_objective_status(&self, obj: &Objective) -> Result<(u8, u8)> {
        obj.groups
            .get(&obj.owner)
            .map(|groups| {
                let mut total = 0;
                let mut alive = 0;
                let mut logi_total = 0;
                let mut logi_alive = 0;
                for (_, gid) in groups {
                    let group = group!(self, gid)?;
                    let logi = match &group.class {
                        ObjGroupClass::Logi => true,
                        _ => false,
                    };
                    for uid in &group.units {
                        total += 1;
                        if logi {
                            logi_total += 1;
                        }
                        if !unit!(self, uid)?.dead {
                            alive += 1;
                            if logi {
                                logi_alive += 1;
                            }
                        }
                    }
                }
                let health = ((alive as f32 / total as f32) * 100.).trunc() as u8;
                let logi = ((logi_alive as f32 / logi_total as f32) * 100.).trunc() as u8;
                Ok((health, logi))
            })
            .unwrap_or(Ok((100, 100)))
    }

    pub(super) fn update_objective_status(
        &mut self,
        oid: &ObjectiveId,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let obj = objective!(self, oid)?;
        let (health, logi) = self.compute_objective_status(obj)?;
        let obj = objective_mut!(self, oid)?;
        obj.health = health;
        obj.logi = logi;
        obj.last_change_ts = now;
        if obj.health == 0 {
            obj.owner = Side::Neutral;
        }
        debug!("objective {oid} health: {}, logi: {}", obj.health, obj.logi);
        Ok(())
    }

    fn repair_objective(
        &mut self,
        idx: &MizIndex,
        spctx: &SpawnCtx,
        oid: ObjectiveId,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let obj = self
            .persisted
            .objectives
            .get(&oid)
            .ok_or_else(|| anyhow!("no such objective {:?}", oid))?;
        if let Some(groups) = obj.groups.get(&obj.owner) {
            let mut damaged_by_class: FxHashMap<ObjGroupClass, Vec<(GroupId, usize)>> =
                groups.into_iter().fold(
                    Ok(FxHashMap::default()),
                    |m: Result<FxHashMap<ObjGroupClass, Vec<(GroupId, usize)>>>, (_, id)| {
                        let mut m = m?;
                        let group = group!(self, id)?;
                        let mut damaged = 0;
                        for uid in &group.units {
                            damaged += if unit!(self, uid)?.dead { 1 } else { 0 };
                        }
                        if damaged > 0 {
                            m.entry(group.class).or_default().push((*id, damaged));
                            Ok(m)
                        } else {
                            Ok(m)
                        }
                    },
                )?;
            for class in [
                ObjGroupClass::Logi,
                ObjGroupClass::Sr,
                ObjGroupClass::Aaa,
                ObjGroupClass::Mr,
                ObjGroupClass::Lr,
                ObjGroupClass::Armor,
                ObjGroupClass::Other,
            ] {
                if let Some(groups) = damaged_by_class.get_mut(&class) {
                    groups.sort_by_key(|(_, d)| *d); // pick the most damaged group
                    if let Some((gid, _)) = groups.pop() {
                        let group = &self.persisted.groups[&gid];
                        for uid in &group.units {
                            self.persisted.units[uid].dead = false;
                        }
                        if class == ObjGroupClass::Logi || obj.spawned {
                            self.spawn_group(idx, spctx, group)?;
                        }
                        self.update_objective_status(&oid, now)?;
                        self.ephemeral.dirty = true;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn cull_or_respawn_objectives(&mut self, lua: MizLua, idx: &MizIndex) -> Result<()> {
        let players = self
            .ephemeral
            .players_by_slot
            .iter()
            .filter_map(|(sl, ucid)| {
                let side = self.persisted.players[ucid].side;
                let pos = self
                    .slot_instance_unit(lua, idx, sl)
                    .and_then(|u| u.as_object()?.get_point())
                    .map(|v| Vector2::new(v.x, v.z));
                match pos {
                    Ok(pos) => Some((side, pos)),
                    Err(e) => {
                        error!(
                            "failed to get position of player {:?} {:?} {:?}",
                            sl, ucid, e
                        );
                        None
                    }
                }
            })
            .collect::<Vec<_>>();
        let cfg = self.cfg();
        let mut to_spawn: SmallVec<[ObjectiveId; 8]> = smallvec![];
        let mut to_cull: SmallVec<[ObjectiveId; 8]> = smallvec![];
        for (oid, obj) in &self.persisted.objectives {
            match obj.owner {
                Side::Blue | Side::Red => (),
                Side::Neutral => continue,
            }
            let mut spawn = false;
            for (side, pos) in &players {
                if obj.owner != *side
                    && na::distance(&obj.pos.into(), &(*pos).into())
                        <= cfg.unit_cull_distance as f64
                {
                    spawn = true;
                    break;
                }
            }
            if !obj.spawned && spawn {
                to_spawn.push(*oid);
            } else if obj.spawned && !spawn {
                to_cull.push(*oid);
            }
        }
        for oid in to_spawn {
            let obj = objective_mut!(self, oid)?;
            obj.spawned = true;
            for (_name, gid) in &obj.groups[&obj.owner] {
                if !group!(self, gid)?.class.is_logi() {
                    self.ephemeral.spawnq.push_back(*gid);
                }
            }
        }
        for oid in to_cull {
            let obj = objective_mut!(self, oid)?;
            obj.spawned = false;
            for (_name, gid) in &obj.groups[&obj.owner] {
                let group = group!(self, gid)?;
                if !group.class.is_logi() {
                    match group.kind {
                        Some(_) => self
                            .ephemeral
                            .despawnq
                            .push_back(Despawn::Group(group.name.clone())),
                        None => {
                            for uid in &group.units {
                                let unit = unit!(self, uid)?;
                                self.ephemeral
                                    .despawnq
                                    .push_back(Despawn::Static(unit.name.clone()))
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn maybe_do_repairs(
        &mut self,
        lua: MizLua,
        idx: &MizIndex,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let spctx = SpawnCtx::new(lua)?;
        let to_repair = self
            .persisted
            .objectives
            .into_iter()
            .filter_map(|(oid, obj)| {
                let logi = obj.logi as f32 / 100.;
                let repair_time = self.ephemeral.cfg.repair_time as f32 / logi;
                if repair_time < i64::MAX as f32 {
                    let repair_time = Duration::seconds(repair_time as i64);
                    if obj.health < 100 && (now - obj.last_change_ts) >= repair_time {
                        Some(*oid)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for oid in to_repair {
            self.repair_objective(idx, &spctx, oid, now)?
        }
        Ok(())
    }
}
