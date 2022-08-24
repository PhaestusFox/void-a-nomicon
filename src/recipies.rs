use std::{collections::HashMap, path::Path};
use crate::prelude::*;

pub struct RecipiePlugin;

impl Plugin for RecipiePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Recipies::default());
        app.add_system(combine);
        app.add_startup_system(load_recipies);
    }
}

fn load_recipies(
    mut recipies: ResMut<Recipies>,
) {
    use std::fs;
    for file in fs::read_dir("./assets/recipies").unwrap() {
        if file.is_err() {continue;}
        let file = file.unwrap();
        if let Some(ext) = file.path().extension() {if ext != "vr" {continue;}}
        recipies.load(file.path());
    }
}



#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::prelude::*;
    use super::{Recipie,Recipies};
    #[test]
    fn item_item_test() {
        let mut recipies = Recipies{
            items: HashMap::default(),
        };
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
        let mut recipies = Recipies{
            items: HashMap::default(),
        };
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
        recipies.load("./assets/recipies/meta.vr");
        let res = recipies.combine(item1, item1);
        assert_eq!(res, Some(vec![item2]));
        let res = recipies.combine(item2, item1);
        assert_eq!(res, Some(vec![item3]));
        let res = recipies.combine(item2, item3);
        assert_eq!(res, Some(vec![item4]));
    }
}

#[derive(Debug, Default)]
pub struct Recipies{
    items: HashMap<ItemID, HashMap<ItemID, (Vec<ItemID>, u16)>>
}

impl Recipies {
    pub fn add(&mut self, recipie: Recipie)
    {
        self.insert(recipie.item_1, recipie.item_2, recipie.result, recipie.priority);
        if recipie.item_1 != recipie.item_2 {
            self.insert(recipie.item_2, recipie.item_1, recipie.result, recipie.priority);
        }
    }

    pub fn load<P>(&mut self, path: P)
    where P: AsRef<Path> {
        let file = std::fs::read_to_string(path).unwrap();
        for line in file.split('\n') {
            self.add(Recipie::from_str(line));
        }
    }

    fn insert(&mut self, outter: ItemID, inner: ItemID, add: ItemID, p: u16) {
        if !self.items.contains_key(&outter) {self.items.insert(outter, HashMap::default());};
        if let Some(out) = self.items.get_mut(&outter) {
            if let Some(inn) = out.get_mut(&inner) {
                if inn.1 > p {
                    out.insert(inner, (vec![add], p));
                } else if inn.1 == p {
                    inn.0.push(add);
                }
            } else {
                out.insert(inner, (vec![add], p));
            }
        }
    } 

    pub fn combine(&self, item1: ItemID, item2: ItemID) -> Option<Vec<ItemID>> {
        if let Some(recipies) = self.items.get(&item1) {
            if let Some(recipie) = recipies.get(&item2) {
                return Some(recipie.0.clone());
            }
        }
        None
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

fn combine(
    mut set: ParamSet<(EventReader<ItemEvent>, EventWriter<ItemEvent>)>,
    recipies: Res<Recipies>,
    query: Query<(&ItemID, &Transform)>,
    mut commands: Commands,
) {
    let mut send = Vec::new();
    for event in set.p0().iter() {
        if let ItemEvent::CheckCombine(item1_e, item2_e) = event {
            let (item1, t1) = if let Ok(i) = query.get(*item1_e) {i} else {continue;};
            let (item2, _) = if let Ok(i) = query.get(*item2_e) {i} else {continue;};
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