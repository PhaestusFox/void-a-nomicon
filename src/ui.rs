use std::collections::HashMap;

use bevy::prelude::*;

pub mod ui_config;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_ui_icons);
        app.init_resource::<UiIcons>();
        app.init_resource::<ui_config::UiConfig>();
    }
}


#[derive(Default)]
pub struct UiIcons{ 
    icons: HashMap<String, Handle<Image>>,
    default: Handle<Image>,
}

impl UiIcons {
    pub fn get(&self, name: &str) -> Handle<Image> {
        if let Some(h) = self.icons.get(name) {
            h.clone()
        } else {
            self.default.clone()
        }
    }
    pub fn load(&mut self, name: &str, path: &str, asset_server: &AssetServer) {
        self.icons.insert(name.to_string(), asset_server.load(path));
    }
}

fn load_ui_icons(
    asset_server: Res<AssetServer>,
    mut icons: ResMut<UiIcons>,
) {
    use std::fs;
    let ui_config = fs::read_to_string("./assets/ui_icons.config").unwrap();
    for line in ui_config.split('\n') {
        let mut line = line.split(':');
        let name = line.next();
        let path = line.next();
        match (name, path) {
            (Some(n), None) => warn!("Ui:{} has no path", n), 
            (Some(n), Some(p)) => icons.load(n.trim(), p.trim(), &asset_server),
            (None, Some(_)) => unreachable!("split should not return None then Some"),
            (None, None) => {},
        }
    }
}
