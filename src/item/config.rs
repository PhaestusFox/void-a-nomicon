use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemConfig{
    pub frame_size: Vec2,
    pub icon_size: Vec2,
}

impl FromWorld for ItemConfig {
    fn from_world(_: &mut World) -> Self {
        if let Ok(res) = load_item_config() {
            res
        } else {
            ItemConfig {
                frame_size: Vec2::splat(100.),
                icon_size: Vec2::splat(90.),
            }
        }
    }
}

fn load_item_config() -> Result<ItemConfig, GameError> {
    let data = std::fs::read_to_string("./assets/item.config")?;
    Ok(ron::from_str(&data)?)
}