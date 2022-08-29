pub mod prelude {
    pub use super::error::GameError;
    pub use crate::ui::ui_config::UiConfig;
    pub use bevy::prelude::*;
    pub use crate::item::ItemID;
    pub use crate::item::ItemEvent;
    pub use crate::item::tags::{Tags, Tag};
    pub use crate::item::physics::Size;
    pub use crate::item::Items;
    pub use crate::MainCam;
}

pub mod error;
pub mod item;
pub mod one_offs;
pub mod ui;
pub mod recipies;
pub mod serde;
pub mod story;
pub mod sound;

#[derive(bevy::prelude::Component)]
pub struct MainCam;