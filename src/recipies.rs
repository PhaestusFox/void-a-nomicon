use std::{collections::{HashMap, HashSet}, path::Path};
use crate::prelude::*;

pub struct RecipiePlugin;

impl Plugin for RecipiePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Recipies::default());
        app.add_system(combine);
        app.add_startup_system(load_recipies);
        app.add_startup_system(set_trash);
        app.init_resource::<MadeSound>();
    }
}

fn load_recipies(
    mut recipies: ResMut<Recipies>,
) {
    use std::fs;
    if let Ok(data) = fs::read_to_string("./assets/made.sav") {
        let mut made = HashSet::new();
        for line in data.split('\n') {
            let id:(ItemID, ItemID) = if let Ok(i) = ron::from_str(line) {i} else {continue;};
            made.insert(id);
        }
        recipies.made = made;
    }
    if let Err(e) = recipies.load_folder("./assets") {
        error!("{}", e);
    }
}

fn set_trash(
    items: Res<Items>,
    mut recipies: ResMut<Recipies>,
) {
    recipies.set_trash(items.all());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude::*;
    use super::{Recipie,Recipies};
    #[test]
    fn item_item_test() {
        let mut recipies = Recipies::default();
        let item1 = ItemID::from("F");
        let item2 = ItemID::from("FF");
        let item3 = ItemID::from("G");
        let item4 = ItemID::from("E");
        recipies.add(Recipie { priority: 0, item_1: item1, item_2: item1, result: item2 });
        recipies.add(Recipie { priority: 0, item_1: item1, item_2: item1, result: item3 });
        recipies.add(Recipie { priority: 0, item_1: item1, item_2: item2, result: item4 });
        recipies.add(Recipie { priority: 1, item_1: item1, item_2: item1, result: item4 });
        let res = recipies.combine(item1, item1);
        assert_eq!(res, Some(vec![item2, item3]));
        let res = recipies.combine(item2, item1);
        assert_eq!(res, Some(vec![item4]));
    }

    #[test]
    fn item_from_str() {
        let mut recipies = Recipies::default();
        let item1 = ItemID::from("F");
        let item2 = ItemID::from("FF");
        let item3 = ItemID::from("G");
        let item4 = ItemID::from("E");
        recipies.add(Recipie::from_str("F + F = FF; 0"));
        recipies.add(Recipie::from_str("F + F = G; 0"));
        recipies.add(Recipie::from_str("F + F = E; 1"));
        recipies.add(Recipie::from_str("F + FF = E; 0"));
        let res = recipies.combine(item1, item1);
        assert_eq!(res, Some(vec![item2, item3]));
        let res = recipies.combine(item2, item1);
        assert_eq!(res, Some(vec![item4]));
    }

    #[test]
    fn load_from_file() {
        let mut recipies = Recipies::default();
        let item1 = ItemID::from("Bevy");
        let item2 = ItemID::from("App");
        let item3 = ItemID::from("DefaultPlugins");
        let item4 = ItemID::from("Totally a game");
        let _ = recipies.load("./assets/recipies/meta.vr");
        let res = recipies.combine(item1, item1);
        assert_eq!(res, Some(vec![item2]));
        let res = recipies.combine(item2, item1);
        assert_eq!(res, Some(vec![item3, item2]));
        let res = recipies.combine(item2, item3);
        assert_eq!(res, Some(vec![item4]));
    }
}

#[derive(Debug, Default)]
pub struct Recipies{
    all: HashMap<ItemID, HashMap<ItemID, (Vec<ItemID>, u16)>>,
    made: HashSet<(ItemID, ItemID)>,
}

impl Recipies {
    pub fn load_folder<P>(&mut self, path: P) -> Result<(), GameError> where P: AsRef<Path> {
        use std::fs;
        for file in fs::read_dir(path)? {
            let file = match file {Ok(f) => {f}, Err(e) => {error!("{}", e); continue;},};
            if file.metadata()?.is_dir() {
                if let Err(e) = self.load_folder(file.path()) {
                    error!("Rec: {}", e);
                };
                continue;
            }
            if let Some(ext) = file.path().extension() {if ext != "vr" {continue;}}
            self.load(file.path())?;
        }
        Ok(())
    }

    #[inline(always)]
    pub fn add(&mut self, recipie: Recipie)
    {
        self.insert(recipie.item_1, recipie.item_2, recipie.result, recipie.priority);
    }

    pub fn save(&self) {
        use std::io::prelude::*;
        let mut file = if let Ok(f) = std::fs::OpenOptions::new().create(true).write(true).open("./assets/made.sav") {
            f
        } else {
            error!("Failed to create file");
            return;
        };
        for item in self.made.iter() {
            if let Ok(d) = ron::to_string(item) {
                let _ = writeln!(&mut file,"{}", d);
            }
        }
    }

    pub fn load<P>(&mut self, path: P) -> Result<(), GameError>
    where P: AsRef<Path> {
        let file = std::fs::read_to_string(path)?;
        for line in file.split('\n') {
            self.add(Recipie::from_str(line));
        }
        Ok(())
    }

    fn insert(&mut self, item1: ItemID, item2: ItemID, add: ItemID, p: u16) {
        let (item1, item2) = item1.first(item2);
        if !self.all.contains_key(&item1) {self.all.insert(item1, HashMap::default());};
        if let Some(out) = self.all.get_mut(&item1) {
            if let Some(inn) = out.get_mut(&item2) {
                if inn.1 > p {
                    out.insert(item2, (vec![add], p));
                } else if inn.1 == p {
                    inn.0.push(add);
                }
            } else {
                out.insert(item2, (vec![add], p));
            }
        }
    } 

    pub fn combine(&mut self, item1: ItemID, item2: ItemID) -> Option<Vec<ItemID>> {
        let (item1, item2) = item1.first(item2);
        if let Some(recipies) = self.all.get(&item1) {
            if let Some(recipie) = recipies.get(&item2) {
                self.made.insert((item1, item2));
                return Some(recipie.0.clone());
            }
        }
        None
    }

    pub fn set_trash(&mut self, items: Vec<ItemID>) {
        let trash = ItemID::from("Trash");
        for item in items {
            self.insert(item, trash, trash, u16::MAX);
        }
    }

    pub fn check_combine(&self, item1: ItemID, item2: ItemID) -> bool {
        let (item1, item2) = item1.first(item2);
        if let Some(recipies) = self.all.get(&item1) {
            recipies.get(&item2).is_some()
        } else {
            false
        }
    }

    pub fn has_made(&self, item1: ItemID, item2: ItemID) -> bool {
        self.made.contains(&item1.first(item2))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Recipie {
    priority: u16,
    item_1: ItemID,
    item_2: ItemID,
    result: ItemID,
}

impl Recipie {
    pub fn from_str(str: &str) -> Recipie {
        let mut chars = str.chars();
        let item1 = extract_word(&mut chars, '+');
        let item2 = extract_word(&mut chars, '=');
        let res = extract_word(&mut chars, ';');
        let p = extract_word(&mut chars, '\n').parse().unwrap_or(0);
        Recipie { priority: p, item_1: ItemID::new(item1), item_2: ItemID::new(item2), result: ItemID::new(res) }
    }

    pub fn items(&self) -> (ItemID, ItemID) {
        self.item_1.first(self.item_2)
    }
}

fn extract_word(chars: &mut std::str::Chars, end: char) -> String {
    let mut word = String::new();
    for c in chars {
        if c == end {break;}
        if c.is_whitespace() {continue;}
        if c == '_' {word.push(' '); continue;}
        word.push(c)
    }
    word
}

struct MadeSound(Handle<AudioSource>);

impl FromWorld for MadeSound {
    fn from_world(world: &mut World) -> Self {
        let a_s = world.resource::<AssetServer>();
        MadeSound(a_s.load("sounds/made.wav"))
    }
}

fn combine(
    mut set: ParamSet<(EventReader<ItemEvent>, EventWriter<ItemEvent>)>,
    mut recipies: ResMut<Recipies>,
    query: Query<(&ItemID, &Transform)>,
    mut commands: Commands,
    res: Res<Audio>,
    made: Res<MadeSound>,
) {
    let trash = ItemID::new("Trash");
    let mut send = Vec::new();
    for event in set.p0().iter() {
        if let ItemEvent::CheckCombine(item1_e, item2_e) = event {
            let (item1, t1) = if let Ok(i) = query.get(*item1_e) {i} else {continue;};
            let (item2, _) = if let Ok(i) = query.get(*item2_e) {i} else {continue;};
            if recipies.has_made(*item1, *item2) && item1.id() != trash.id() && item2.id() != trash.id() {
                res.play(made.0.clone());
                continue;
            }
            if let Some(r) = recipies.combine(*item1, *item2) {
                for r in r {
                    use rand::Rng;
                    let x = rand::thread_rng().gen_range(-50.0..50.0);
                    let y = rand::thread_rng().gen_range(-50.0..50.0);
                    send.push(ItemEvent::SpawnAt(r, Vec3::new(x + t1.translation.x, y + t1.translation.y, 0.0)));
                }
                commands.entity(*item1_e).despawn_recursive();
                commands.entity(*item2_e).despawn_recursive();
            }
        }
    }
    for item in send {
        set.p1().send(item);
    }
}