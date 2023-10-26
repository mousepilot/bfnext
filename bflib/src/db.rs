extern crate nalgebra as na;
use compact_str::format_compact;
use dcso3::{
    coalition::{Coalition, Side},
    cvt_err,
    env::miz::{GroupInfo, GroupKind, Miz, MizIndex, TriggerZone, TriggerZoneTyp, Group},
    err,
    group::GroupCategory,
    DeepClone, String, Vector2,
};
use fxhash::FxHashMap;
use mlua::{prelude::*, Value};
use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    path::{Path, PathBuf},
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};

type Map<K, V> = immutable_chunkmap::map::Map<K, V, 32>;
type Set<K> = immutable_chunkmap::set::Set<K, 32>;

macro_rules! atomic_id {
    ($name:ident) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
            pub struct $name(u64);

            static [<MAX_ $name:upper _ID>]: AtomicU64 = AtomicU64::new(0);

            impl Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl Default for $name {
                fn default() -> Self {
                    Self(0)
                }
            }

            impl $name {
                pub fn new() -> Self {
                    Self([<MAX_ $name:upper _ID>].fetch_add(1, Ordering::Relaxed))
                }

                fn update_max(id: Self) {
                    // not strictly thread safe, but it doesn't matter in this context
                    if id.0 >= [<MAX_ $name:upper _ID>].load(Ordering::Relaxed) {
                        [<MAX_ $name:upper _ID>].store(id.0 + 1, Ordering::Relaxed)
                    }
                }
            }
        }
    }
}

atomic_id!(GroupId);
atomic_id!(UnitId);
atomic_id!(ObjectiveId);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnedUnit {
    pub name: String,
    pub id: UnitId,
    pub group: GroupId,
    pub template_name: String,
    pub pos: Vector2,
    pub dead: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedGroup {
    pub id: GroupId,
    pub name: String,
    pub template_name: String,
    pub side: Side,
    pub kind: GroupKind,
    pub units: Set<UnitId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnLoc {
    AtPos(Vector2),
    AtTrigger { name: String, offset: Vector2 },
}

pub struct SpawnCtx<'lua> {
    coalition: Coalition<'lua>,
    miz: Miz<'lua>,
    lua: &'lua Lua,
}

impl<'lua> SpawnCtx<'lua> {
    pub fn new(lua: &'lua Lua) -> LuaResult<Self> {
        Ok(Self {
            coalition: Coalition::singleton(lua)?,
            miz: Miz::singleton(lua)?,
            lua,
        })
    }

    pub fn get_template(
        &self,
        idx: &MizIndex,
        kind: GroupKind,
        side: Side,
        template_name: &str,
    ) -> LuaResult<GroupInfo> {
        let mut template = self
            .miz
            .get_group(idx, kind, side, template_name)?
            .ok_or_else(|| err("no such template"))?;
        template.group = template.group.deep_clone(self.lua)?;
        Ok(template)
    }

    pub fn get_trigger_zone(&self, idx: &MizIndex, name: &str) -> LuaResult<TriggerZone> {
        Ok(self
            .miz
            .get_trigger_zone(idx, name)?
            .ok_or_else(|| err("no such trigger zone"))?)
    }

    pub fn spawn(&self, template: GroupInfo) -> LuaResult<()> {
        match GroupCategory::from_kind(template.category) {
            None => self
                .coalition
                .add_static_object(template.country, template.group),
            Some(category) => self
                .coalition
                .add_group(template.country, category, template.group),
        }
    }

    pub fn despawn(&self, name: &str) -> LuaResult<()> {
        let group = dcso3::group::Group::get_by_name(&self.lua, name)?;
        group.destroy()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveKind {
    Airbase,
    Fob,
    Fuelbase,
    Samsite,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ObjGroup(String);

impl FromStr for ObjGroup {
    type Err = LuaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("#") {
            None => Err(err("expected a #")),
            Some((_, _)) => Ok(Self(String::from(s))),
        }
    }
}

impl<'lua> FromLua<'lua> for ObjGroup {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            Value::String(s) => s.to_str()?.parse(),
            _ => Err(cvt_err("ObjGroup")),
        }
    }
}

impl ObjGroup {
    fn template(&self) -> &str {
        self.0.split_once("#").unwrap().0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    id: ObjectiveId,
    spawned: bool,
    trigger_name: String,
    name: String,
    pos: Vector2,
    radius: f64,
    owner: Side,
    kind: ObjectiveKind,
    slots: Set<String>,
    groups: Map<Side, Map<ObjGroup, GroupId>>,
    logistics: Set<ObjGroup>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Db {
    #[serde(skip)]
    dirty: bool,
    groups_by_id: Map<GroupId, SpawnedGroup>,
    units_by_id: Map<UnitId, SpawnedUnit>,
    groups_by_name: Map<String, GroupId>,
    units_by_name: Map<String, UnitId>,
    groups_by_side: Map<Side, Set<GroupId>>,
    objectives: Map<ObjectiveId, Objective>,
    objectives_by_slot: Map<String, ObjectiveId>,
    objectives_by_name: Map<String, ObjectiveId>,
    objectives_by_group: Map<GroupId, ObjectiveId>,
}

impl Db {
    pub fn load(path: &Path) -> LuaResult<Self> {
        let file = File::open(&path).map_err(|e| {
            println!("failed to open save file {:?}, {:?}", path, e);
            err("io error")
        })?;
        let db: Self = serde_json::from_reader(file).map_err(|e| {
            println!("failed to decode save file {:?}, {:?}", path, e);
            err("decode error")
        })?;
        for (id, _) in &db.groups_by_id {
            GroupId::update_max(*id)
        }
        for (id, _) in &db.units_by_id {
            UnitId::update_max(*id)
        }
        for (id, _) in &db.objectives {
            ObjectiveId::update_max(*id)
        }
        Ok(db)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let mut tmp = PathBuf::from(path);
        tmp.set_extension("tmp");
        let file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&tmp)?;
        serde_json::to_writer(file, self)?;
        fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn maybe_snapshot(&mut self) -> Option<Self> {
        if self.dirty {
            self.dirty = false;
            Some(self.clone())
        } else {
            None
        }
    }

    /// objectives are just trigger zones named according to type codes
    /// the first caracter is the type of the zone
    /// O - Objective
    /// G - Group within an objective
    /// T - Generic trigger zone, ignored by the engine
    ///
    /// Then a 2 character type code
    /// - AB: Airbase
    /// - FO: Fob
    /// - SA: Sam site
    /// - FB: Fuel base
    ///
    /// Then a 1 character code for the default owner
    /// followed by the display name
    /// - R: Red
    /// - B: Blue
    /// - N: Neutral
    ///
    /// So e.g. Tblisi would be OABBTBLISI -> Objective, Airbase, Default to Blue, named Tblisi
    fn init_objective(&mut self, zone: TriggerZone, name: &str) -> LuaResult<()> {
        fn side_and_name(s: &str) -> LuaResult<(Side, String)> {
            if let Some(name) = s.strip_prefix("R") {
                Ok((Side::Red, String::from(name)))
            } else if let Some(name) = s.strip_prefix("B") {
                Ok((Side::Blue, String::from(name)))
            } else if let Some(name) = s.strip_prefix("N") {
                Ok((Side::Neutral, String::from(name)))
            } else {
                Err(err("invalid default coalition, expected B, R, or N"))
            }
        }
        let (kind, owner, name) = if let Some(name) = name.strip_prefix("AB") {
            let (side, name) = side_and_name(name)?;
            (ObjectiveKind::Airbase, side, name)
        } else if let Some(name) = name.strip_prefix("FO") {
            let (side, name) = side_and_name(name)?;
            (ObjectiveKind::Fob, side, name)
        } else if let Some(name) = name.strip_prefix("FB") {
            let (side, name) = side_and_name(name)?;
            (ObjectiveKind::Fuelbase, side, name)
        } else if let Some(name) = name.strip_prefix("SA") {
            let (side, name) = side_and_name(name)?;
            (ObjectiveKind::Samsite, side, name)
        } else {
            return Err(err("invalid objective type"));
        };
        let id = ObjectiveId::new();
        let radius = match zone.typ()? {
            TriggerZoneTyp::Quad(_) => return Err(err("invalid zone volume type")),
            TriggerZoneTyp::Circle { radius } => radius,
        };
        let pos = zone.pos()?;
        let obj = Objective {
            id,
            spawned: false,
            trigger_name: zone.name()?,
            pos,
            radius,
            name: name.clone(),
            kind,
            owner,
            slots: Set::new(),
            groups: Map::new(),
            logistics: Set::new(),
        };
        self.objectives.insert_cow(id, obj);
        self.objectives_by_name.insert_cow(name, id);
        Ok(())
    }

    /// Objective groups are trigger zones with the first character set to G. They are then a template
    /// name, followed by # and a number. They are associated with an objective by proximity.
    /// e.g. GRIRSRAD#001 would be the 1st instantiation of the template RIRSRAD, which must
    /// correspond to a group in the miz file. There is one special template name called (R|B|N)LOGI
    /// which corresponds to the logistics template for objectives
    fn init_objective_group(
        &mut self,
        spctx: &SpawnCtx,
        idx: &MizIndex,
        _miz: &Miz,
        zone: TriggerZone,
        name: &str,
    ) -> LuaResult<()> {
        let name = name.parse::<ObjGroup>()?;
        let pos = zone.pos()?;
        let (obj, side) = {
            let mut iter = self.objectives.into_iter();
            loop {
                match iter.next() {
                    None => return Err(err("group isn't associated with an objective")),
                    Some((id, obj)) => {
                        // this is inefficent; look into an orthographic database
                        if na::distance(&pos.into(), &obj.pos.into()) <= obj.radius {
                            break (*id, obj.owner);
                        }
                    }
                }
            }
        };
        let (gid, _) = self.init_template(
            spctx,
            idx,
            side,
            GroupKind::Any,
            &SpawnLoc::AtPos(pos),
            name.template(),
        )?;
        self.objectives[&obj]
            .groups
            .get_or_default_cow(side)
            .insert_cow(name.clone(), gid);
        self.objectives_by_group.insert_cow(gid, obj);
        let logi = match side {
            Side::Blue => name.template() == "BLOGI",
            Side::Red => name.template() == "RLOGI",
            Side::Neutral => name.template() == "NLOGI",
        };
        if logi {
            self.objectives[&obj].logistics.insert_cow(name);
        }
        Ok(())
    }

    pub fn init_objective_slot(&mut self, slot: Group) -> LuaResult<()> {
        let name = slot.name()?;
        let pos = slot.pos()?;
        let obj = {
            let mut iter = self.objectives.into_iter();
            loop {
                match iter.next() {
                    None => return Err(err("slot not associated with an objective")),
                    Some((id, obj)) => {
                        if na::distance(&pos.into(), &obj.pos.into()) <= obj.radius {
                            break *id
                        }
                    }
                }
            }
        };
        self.objectives_by_slot.insert_cow(name.clone(), obj);
        self.objectives[&obj].slots.insert_cow(name);
        Ok(())
    }

    pub fn init(lua: &Lua, idx: &MizIndex, miz: &Miz) -> LuaResult<Self> {
        let spctx = SpawnCtx::new(lua)?;
        let mut t = Self::default();
        // first init all the objectives
        for zone in miz.triggers()? {
            let zone = zone?;
            let name = zone.name()?;
            if let Some(name) = name.strip_prefix("O") {
                t.init_objective(zone, name)?
            }
        }
        // now associate groups with objectives
        for zone in miz.triggers()? {
            let zone = zone?;
            let name = zone.name()?;
            if let Some(name) = name.strip_prefix("G") {
                t.init_objective_group(&spctx, idx, miz, zone, name)?
            } else if name.starts_with("T") || name.starts_with("O") {
                () // ignored
            } else {
                return Err(err("invalid trigger zone type code, expected O, G, or T"));
            }
        }
        // now associate slots with objectives
        for coa in [
            miz.coalition(Side::Blue)?,
            miz.coalition(Side::Red)?,
            miz.coalition(Side::Neutral)?,
        ] {
            for country in coa.countries()? {
                let country = country?;
                for plane in country.planes()? {
                    let plane = plane?;
                    t.init_objective_slot(plane)?
                }
                for heli in country.helicopters()? {
                    let heli = heli?;
                    t.init_objective_slot(heli)?
                }
            }
        }
        t.dirty = true;
        println!("{:#?}", &t);
        Ok(t)
    }

    pub fn unit_dead(&mut self, id: UnitId, dead: bool) {
        if let Some(unit) = self.units_by_id.get_mut_cow(&id) {
            unit.dead = dead;
        }
        self.dirty = true;
    }

    pub fn groups(&self) -> impl Iterator<Item = (&GroupId, &SpawnedGroup)> {
        self.groups_by_id.into_iter()
    }

    pub fn get_group(&self, id: &GroupId) -> Option<&SpawnedGroup> {
        self.groups_by_id.get(id)
    }

    pub fn get_group_by_name(&self, name: &str) -> Option<&SpawnedGroup> {
        self.groups_by_name
            .get(name)
            .and_then(|gid| self.groups_by_id.get(gid))
    }

    pub fn get_unit(&self, id: &UnitId) -> Option<&SpawnedUnit> {
        self.units_by_id.get(id)
    }

    pub fn get_unit_by_name(&self, name: &str) -> Option<&SpawnedUnit> {
        self.units_by_name
            .get(name)
            .and_then(|uid| self.get_unit(uid))
    }

    pub fn respawn_group<'lua>(
        &self,
        idx: &MizIndex,
        spctx: &SpawnCtx,
        group: &SpawnedGroup,
    ) -> LuaResult<()> {
        let template =
            spctx.get_template(idx, group.kind, group.side, group.template_name.as_str())?;
        template.group.set("lateActivation", false)?;
        template.group.set_name(group.name.clone())?;
        let by_tname: FxHashMap<&str, &SpawnedUnit> = group
            .units
            .into_iter()
            .filter_map(|uid| {
                self.units_by_id.get(uid).and_then(|u| {
                    if u.dead {
                        None
                    } else {
                        Some((u.template_name.as_str(), u))
                    }
                })
            })
            .collect();
        let alive = {
            let units = template.group.units()?;
            let mut i = 1;
            while i as usize <= units.len() {
                let unit = units.get(i)?;
                match by_tname.get(unit.name()?.as_str()) {
                    None => units.remove(i)?,
                    Some(su) => {
                        template.group.set_pos(su.pos)?;
                        unit.set_pos(su.pos)?;
                        unit.set_name(su.name.clone())?;
                        i += 1;
                    }
                }
            }
            units.len() > 0
        };
        if alive {
            spctx.spawn(template)
        } else {
            Ok(())
        }
    }

    /// add the units to the db, but don't actually spawn them
    fn init_template<'lua>(
        &mut self,
        spctx: &'lua SpawnCtx<'lua>,
        idx: &MizIndex,
        side: Side,
        kind: GroupKind,
        location: &SpawnLoc,
        template_name: &str,
    ) -> LuaResult<(GroupId, GroupInfo<'lua>)> {
        let template_name = String::from(template_name);
        let template = spctx.get_template(idx, kind, side, template_name.as_str())?;
        let pos = match location {
            SpawnLoc::AtPos(pos) => *pos,
            SpawnLoc::AtTrigger { name, offset } => {
                spctx.get_trigger_zone(idx, name.as_str())?.pos()? + offset
            }
        };
        let gid = GroupId::new();
        let group_name = String::from(format_compact!("{}-{}", template_name, gid));
        template.group.set("lateActivation", false)?;
        template.group.raw_remove("groupId")?;
        let orig_group_pos = template.group.pos()?;
        template.group.set_pos(pos)?;
        template.group.set_name(group_name.clone())?;
        let mut spawned = SpawnedGroup {
            id: gid,
            name: group_name.clone(),
            template_name: template_name.clone(),
            side,
            kind,
            units: Set::new(),
        };
        for unit in template.group.units()? {
            let uid = UnitId::new();
            let unit = unit?;
            let template_name = unit.name()?;
            let unit_name = String::from(format_compact!("{}-{}", group_name, uid));
            let unit_pos_offset = orig_group_pos - unit.pos()?;
            let pos = pos + unit_pos_offset;
            unit.raw_remove("unitId")?;
            unit.set_pos(pos)?;
            unit.set_name(unit_name.clone())?;
            let spawned_unit = SpawnedUnit {
                id: uid,
                group: gid,
                name: unit_name.clone(),
                template_name,
                pos,
                dead: false,
            };
            spawned.units.insert_cow(uid);
            self.units_by_id.insert_cow(uid, spawned_unit);
            self.units_by_name.insert_cow(unit_name, uid);
        }
        self.groups_by_id.insert_cow(gid, spawned);
        self.groups_by_name.insert_cow(group_name, gid);
        self.groups_by_side.get_or_default_cow(side).insert_cow(gid);
        Ok((gid, template))
    }

    pub fn spawn_template_as_new<'lua>(
        &mut self,
        lua: &'lua Lua,
        idx: &MizIndex,
        side: Side,
        kind: GroupKind,
        location: &SpawnLoc,
        template_name: &str,
    ) -> LuaResult<GroupId> {
        let spctx = SpawnCtx::new(lua)?;
        let (gid, template) =
            self.init_template(&spctx, idx, side, kind, location, template_name)?;
        self.dirty = true;
        spctx.spawn(template)?;
        Ok(gid)
    }
}
