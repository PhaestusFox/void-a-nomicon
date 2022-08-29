use crate::prelude::*;
use std::{collections::{HashMap, HashSet}, path::Path};

use super::{ItemData, Item, ItemID};

const ITEM_SAVE: &'static str = "./assets/found.sav";

pub struct Items {
    data: HashMap<ItemID, ItemData>,
    debug_item: ItemData,
    found: HashSet<ItemID>,
}

impl FromWorld for Items {
    fn from_world(world: &mut World) -> Self {
        use std::fs;
        let asset_server = world.resource::<AssetServer>();
        let found = if let Ok(str) = fs::read_to_string(ITEM_SAVE) {
            let mut set = HashSet::default();
            for line in str.lines() {
                let line = line.trim();
                let line = if line.starts_with(']') {&line[1..].trim()} else {line};
                let line = if line.ends_with(']') {&line[..line.len()-1].trim()} else {line};
                if line.len() < 1 {continue;}
                set.insert(ItemID::new(line));
            }
            set
        } else {HashSet::default()};
        let mut items = Items {
            data: HashMap::default(),
            debug_item: ItemData {
                name: "Debug Item".to_string(),
                icon: asset_server.load("ui/skull_01.png"),
                tags: Tags::default(),
                description: "This Item Is Spawned in place of an unknown item. maybe you removed a mod? or updated the game".to_string(),
                sound: asset_server.load("sounds/pop.wav"),
            },
            found,
        };
        if let Err(e) = items.load_folder("./assets", &asset_server) {
            error!("{}", e);
        }
        items
    }
}

impl Items {
    pub fn get(&self, id: &ItemID) -> Item {
        if let Some(data) = self.data.get(id) {
            data.into()
        } else {
            Item {
                name: &self.debug_item.name,
                icon: &self.debug_item.icon,
                description: &self.debug_item.description,
                sound: &self.debug_item.sound,
            }
        }
    }

    pub fn found(&self) -> Vec<ItemID> {
        self.found.iter().map(|i| *i).collect()
    }

    pub fn load_folder<P>(&mut self, path: P, asset_server: &AssetServer) -> Result<(), GameError> where P: AsRef<Path> {
        use std::fs;
        for file in fs::read_dir(path)? {
            let file = match file {Ok(f) => {f}, Err(e) => {error!("file: {}", e); continue;},};
            if file.metadata()?.is_dir() {
                if let Err(e) = self.load_folder(file.path(), asset_server) {
                    error!("Rec: {}", e);
                };
                continue;
            }
            if let Some(ext) = file.path().extension() {if ext != "vi" {continue;}}
            if let Err(e) = self.load(file.path(), asset_server) {error!("load: {}", e)};
        }
        Ok(())
    }

    pub fn save_found(&self) -> Result<(), GameError> {
        use std::fs;
        use std::io::prelude::*;
        let mut file = fs::OpenOptions::new().create(true).write(true).truncate(true).open(ITEM_SAVE)?;
        writeln!(&mut file, "[")?;
        for item in self.found.iter() {
            let item = self.get(item);
            writeln!(&mut file, "{}", item.name.replace(' ', "_"))?;
        }
        write!(&mut file, "]")?;
        Ok(())
    }

    pub fn insert(&mut self, id: impl Into<ItemID>, data: ItemData) {
        self.data.insert(id.into(), data);
    }

    pub fn load<P>(&mut self, path: P, asset_server: &AssetServer) -> Result<(), GameError>
    where P: AsRef<std::path::Path>
    {
        let data = std::fs::read_to_string(path)?;
        for item in data.split("{next}") {
            let mut map: HashMap<&str, &str> = HashMap::default();
            for seg in item.split('\n') {
                let mut seg = seg.split(':');
                let name = seg.next();
                let val = seg.next();
                if let (Some(name), Some(val)) = (name, val) {
                    map.insert(name.trim(), val.trim());
                } else {
                    debug!("failed to load {:?} with {:?}; {}:{}:{}", name, val, file!(), line!(), column!());
                }
            }
            let icon_path: String = unwrap_or_t(&map, "icon")?;
            let sound: String = unwrap_or_t(&map, "sound").unwrap_or("sounds/pop.wav".to_string());
            let name: String = unwrap_or_t(&map, "name")?;
            let id = ItemID::from(name.as_str());
            let tags = unwrap_or_t(&map, "tags").unwrap_or_default();
            let description = unwrap_or_t(&map, "description").unwrap_or("No description for this item;".to_string());
            self.insert(id, ItemData {
                name,
                icon: asset_server.load(&icon_path),
                tags,
                description,
                sound: asset_server.load(&sound),
            });
        }
        Ok(())
    }

    pub fn all(&self) -> Vec<ItemID> {
        self.data.keys().map(|i| *i).collect()
    }
}

pub fn found_update(
    mut set: ParamSet<(EventReader<ItemEvent>,EventWriter<ItemEvent>)>,
    mut items: ResMut<Items>,
) {
    let mut found = Vec::new();
    for event in set.p0().iter() {
        match event {
            ItemEvent::Spawn(id) |
            ItemEvent::SpawnAt(id, _) => {
                if !items.found.contains(id) {
                    items.found.insert(*id);
                    found.push(*id);
                }
            }
            _ => {},
        }
    }
    for found in found {
        set.p1().send(ItemEvent::Found(found));
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

