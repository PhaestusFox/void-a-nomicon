use std::{path::PathBuf, str::Chars};

use bevy::time::FixedTimestep;

use crate::{prelude::*, item::Items};

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
        app.add_startup_system(load);
    }
}

fn save(
    world: Query<(&ItemID, &Transform)>,
    items: Res<Items>,
) {
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
}

fn save_to_file(file: &mut std::fs::File, query: &Query<(&ItemID, &Transform)>, items: &Items) -> Result<(), GameError> {
    use std::io::prelude::*;
    for (item, at) in query.iter() {
        let name = items.get(item).name().replace(' ', "_");
        writeln!(file, "{}:{}",name, at.translation.truncate())?;
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
    info!("x:{}; y:{};", x , y);
    let x = x.parse()?;
    let y = y.parse()?;
    Ok((ItemID::new(name), Vec2::new(x,y)))
}