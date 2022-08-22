use crate::prelude::*;
use std::{collections::HashMap};

use super::{ItemData, Item, ItemID};

pub struct Items {
    data: HashMap<ItemID, ItemData>,
    debug_item: ItemData,
}

impl FromWorld for Items {
    fn from_world(world: &mut World) -> Self {
        let assetserver = world.resource::<AssetServer>();
        Items {
            data: HashMap::default(),
            debug_item: ItemData { name: "Debug Item".to_string(), icon: assetserver.load("icon.png") }
        }
    }
}

impl Items {
    pub fn get(&self, id: &ItemID) -> Item {
        if let Some(data) = self.data.get(id) {
            data.into()
        } else {
            Item { name: &self.debug_item.name, icon: self.debug_item.icon.clone() }
        }
    }

    pub fn insert(&mut self, id: impl Into<ItemID>, data: ItemData) {
        self.data.insert(id.into(), data);
    }

    pub fn load<P>(&mut self, path: P, asset_server: &AssetServer) -> Result<(), GameError>
    where P: AsRef<std::path::Path>
    {
        let data = std::fs::read_to_string(path)?;
        let mut map: HashMap<&str, &str> = HashMap::default();
        for seg in data.split('\n') {
            let mut seg = seg.split(':');
            let name = seg.next();
            let val = seg.next();
            if let (Some(name), Some(val)) = (name, val) {
                map.insert(name, val);
            } else {
                debug!("failed to load {:?} with {:?}; {}:{}:{}", name, val, file!(), line!(), column!());
            }
        }
        let icon_path: String = unwrap_or_t(&map, "icon")?;
        let name: String = unwrap_or_t(&map, "name")?;
        let id = ItemID::from(name.as_str());
        self.insert(id, ItemData {
            name,
            icon: asset_server.load(&icon_path),
        });
        Ok(())
    }
}

fn unwrap_or_404<'de, 'a>(map: &'de HashMap<&str, &'a str>, field: &str) -> Result<&'a str, GameError> {
    if let Some(v) = map.get(field) {
        Ok(*v)
    } else {
        Err(GameError::FieldNotFound(field.to_string()))
    }
}

fn unwrap_or_t<'de, T>(map: &'de HashMap<&str, &str>, field: &str) -> Result<T, GameError>
    where T: serde::Deserialize<'de>,
{
    if let Some(v) = map.get(field) {
        Ok(ron::from_str(v)?)
    } else {
        Err(GameError::FieldNotFound(field.to_string()))
    }
}

