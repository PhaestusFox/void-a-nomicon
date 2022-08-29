use std::{path::PathBuf, str::Chars};

use bevy::{time::FixedTimestep, window::WindowCloseRequested};

use crate::{prelude::*, item::Items, recipies::Recipies, ui};

pub struct SaveLoadPlugin;

const SAVE_PATH: &'static str = "./assets/game.sav";
const OLD_PATH: &'static str = "./assets/game.old";

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(30.0))
            .with_system(save)
        );
        app.add_system(save_on_quit);
        app.add_startup_system(load);
    }
}

fn save_on_quit(
    world: Query<(&ItemID, &Transform), Without<ui::ItemSpaceItem>>,
    items: Res<Items>,
    mut events: EventReader<WindowCloseRequested>,
    made: Res<Recipies>,
) {
    for event in events.iter() {
        if event.id.is_primary() {
            use std::fs;
            if PathBuf::from(SAVE_PATH).exists() {
                fs::rename(&SAVE_PATH, &OLD_PATH).expect("can rename");
            }
            let mut file = match fs::OpenOptions::new().write(true).create(true).open("./assets/game.sav") {
            Ok(f) => {f},
            Err(e) => {
                if PathBuf::from(OLD_PATH).exists() {
                    fs::rename(&OLD_PATH, &SAVE_PATH).expect("can rename");
                }
                error!("{}",e); return;}
            };
            if let Err(e) = save_to_file(&mut file, &world, &items) {
                if PathBuf::from(OLD_PATH).exists() {
                    fs::rename(&OLD_PATH, &SAVE_PATH).expect("can rename");
                }
                error!("{}",e); return;
            }
            made.save();
            if let Err(e) = items.save_found() {
                error!("{}", e);
            }
        }
    }
}

fn save(
    world: Query<(&ItemID, &Transform), Without<ui::ItemSpaceItem>>,
    items: Res<Items>,
) {
    use std::fs;
    if PathBuf::from(SAVE_PATH).exists() {
        fs::rename(&SAVE_PATH, &OLD_PATH).expect("can rename");
    }
    let mut file = match fs::OpenOptions::new().write(true).create(true).open(SAVE_PATH) {
      Ok(f) => {f},
      Err(e) => {
        if PathBuf::from(OLD_PATH).exists() {
            fs::rename(&OLD_PATH, &SAVE_PATH).expect("can rename");
        }
        error!("{}",e); return;}
    };
    if let Err(e) = save_to_file(&mut file, &world, &items) {
        if PathBuf::from(OLD_PATH).exists() {
            fs::rename(&OLD_PATH, &SAVE_PATH).expect("can rename");
        }
        error!("{}",e); return;
    }
}

fn save_to_file(file: &mut std::fs::File, query: &Query<(&ItemID, &Transform), Without<ui::ItemSpaceItem>>, items: &Items) -> Result<(), GameError> {
    use std::io::prelude::*;
    let trash = ItemID::new("Trash");
    let app = ItemID::new("Totally a game");
    let debug = ItemID::new("Debug Item");
    for (item, at) in query.iter() {
        if item == &trash {continue;
        } else if item == &app {continue;
        } else if item == &debug {continue;
        } else {
            let name = items.get(item).name().replace(' ', "_");
            writeln!(file, "{}:{}",name, at.translation.truncate())?;
        }
    }
    Ok(())
}

fn load(
    mut events: EventWriter<ItemEvent>,
) {
    if let Err(e) = if PathBuf::from(SAVE_PATH).exists() {
         load_path(&SAVE_PATH, &mut events)
    } else if PathBuf::from(OLD_PATH).exists() {
        load_path(&OLD_PATH, &mut events)
    } else {
        info!("Failed to find save");
        Ok(())
    } {
        error!("{}", e);
    }
}

fn load_path(path: &str, events: &mut EventWriter<ItemEvent>) -> Result<(), GameError> {
    use std::fs;
    let data = fs::read_to_string(path)?;
    for line in data.split('\n') {
        if line.len() < 6 {continue;}
        let (id, at) = extract_item(line.chars())?;
        events.send(ItemEvent::SpawnAt(id, at.extend(0.0)));
    }   
    Ok(())
}

fn extract_item(mut chars: Chars) -> Result<(ItemID, Vec2), GameError> {
    let mut name = String::new();
    while let Some(c) = chars.next() {
        if c == ':' {break;}
        if c == '_' {name.push(' '); continue;}
        if c.is_whitespace() {continue;}
        name.push(c);
    };
    let mut open = false;
    let mut x = String::new();
    let mut y = String::new();
    let mut is_x = true;
    while let Some(c) = chars.next() {
        if c.is_whitespace() {continue;}
        if c == '[' {open = true; continue;}
        if c == ']' {break;}
        if !open {return Err(GameError::WrongChar(c, '['));}
        if c == ',' {is_x = false; continue;}
        if is_x {x.push(c)} else {y.push(c)}
    }
    let x = x.parse()?;
    let y = y.parse()?;
    Ok((ItemID::new(name), Vec2::new(x,y)))
}