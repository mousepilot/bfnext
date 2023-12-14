use super::{as_tbl, event::Event, unit::Unit, String};
use crate::{
    airbase::Airbase,
    atomic_id,
    object::{Object, ObjectCategory},
    wrap_bool, wrapped_table, LuaEnv, LuaVec3, MizLua, Position3, Sequence,
};
use anyhow::Result;
use compact_str::format_compact;
use log::{error, warn};
use mlua::{prelude::*, Value};
use serde_derive::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SearchVolume {
    Segment {
        from: LuaVec3,
        to: LuaVec3,
    },
    Box {
        min: LuaVec3,
        max: LuaVec3,
    },
    Sphere {
        point: LuaVec3,
        radius: f64,
    },
    Pyramid {
        pos: Position3,
        length: f32,
        half_angle_hor: f32,
        half_angle_ver: f32,
    },
}

impl<'lua> IntoLua<'lua> for SearchVolume {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<Value<'lua>> {
        let tbl = lua.create_table()?;
        let params = lua.create_table()?;
        match self {
            Self::Segment { from, to } => {
                tbl.raw_set("id", 0)?;
                params.raw_set("from", from)?;
                params.raw_set("to", to)?;
            }
            Self::Box { min, max } => {
                tbl.raw_set("id", 1)?;
                params.raw_set("min", min)?;
                params.raw_set("max", max)?;
            }
            Self::Sphere { point, radius } => {
                tbl.raw_set("id", 2)?;
                params.raw_set("point", point)?;
                params.raw_set("radius", radius)?;
            }
            Self::Pyramid {
                pos,
                length,
                half_angle_hor,
                half_angle_ver,
            } => {
                tbl.raw_set("id", 3)?;
                params.raw_set("pos", pos)?;
                params.raw_set("length", length)?;
                params.raw_set("halfAngleHor", half_angle_hor)?;
                params.raw_set("halfAngleVer", half_angle_ver)?;
            }
        }
        tbl.raw_set("params", params)?;
        Ok(Value::Table(tbl))
    }
}

atomic_id!(HandlerId);

impl HandlerId {
    fn key(&self) -> String {
        String(format_compact!("rustHandler{}", self.0))
    }
}

wrapped_table!(World, None);

impl<'lua> World<'lua> {
    pub fn singleton(lua: MizLua<'lua>) -> Result<Self> {
        Ok(lua.inner().globals().raw_get("world")?)
    }

    pub fn add_event_handler<F>(&self, f: F) -> Result<HandlerId>
    where
        F: Fn(MizLua<'lua>, Event) -> Result<()> + 'static,
    {
        let globals = self.lua.globals();
        let id = HandlerId::new();
        let tbl = self.lua.create_table()?;
        tbl.set(
            "onEvent",
            self.lua
                .create_function(move |lua, (_, ev): (Value, Value)| {
                    match Event::from_lua(ev, lua) {
                        Err(e) => {
                            warn!("error translating event: {:?}", e);
                            Ok(())
                        }
                        Ok(ev) => match f(MizLua(lua), ev) {
                            Ok(()) => Ok(()),
                            Err(e) => {
                                error!("error in event handler: {:?}", e);
                                Ok(())
                            }
                        },
                    }
                })?,
        )?;
        self.t.call_function("addEventHandler", tbl.clone())?;
        globals.raw_set(id.key(), tbl)?;
        Ok(id)
    }

    pub fn remove_event_handler(&self, id: HandlerId) -> Result<()> {
        let globals = self.lua.globals();
        let key = id.key();
        let handler = globals.raw_get(key.clone())?;
        let handler = as_tbl("EventHandler", None, handler)?;
        self.t.call_function("removeEventHandler", handler)?;
        globals.raw_remove(key)?;
        Ok(())
    }

    pub fn get_player(&self) -> Result<Sequence<Unit>> {
        Ok(self.t.call_function("getPlayer", ())?)
    }

    pub fn get_airbases(&self) -> Result<Sequence<Airbase>> {
        Ok(self.t.call_function("getAirbases", ())?)
    }

    pub fn search_objects<F, T>(
        &self,
        category: ObjectCategory,
        volume: SearchVolume,
        f: F,
        arg: Option<T>,
    ) -> Result<()>
    where
        T: IntoLua<'lua> + FromLua<'lua>,
        F: Fn(MizLua, Object, Option<T>) -> Result<bool> + 'static,
    {
        let f = self.lua.create_function(move |lua, (o, arg)| {
            wrap_bool("searchObjects", f(MizLua(lua), o, arg))
        })?;
        Ok(self
            .t
            .call_function("searchObjects", (category, volume, f, arg))?)
    }

    pub fn remove_junk(&self, volume: SearchVolume) -> Result<i64> {
        Ok(self.t.call_function("removeJunk", volume)?)
    }
}
