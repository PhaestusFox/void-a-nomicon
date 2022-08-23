use crate::prelude::*;

mod config;
mod items;
mod event;
mod physics;
mod pickup;

pub use items::Items;
pub use event::ItemEvent;

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemEvent>();
        app.init_resource::<Items>();
        app.init_resource::<config::ItemConfig>();
        app.add_system(spawn_item);
        app.add_system(event::move_down);
        app.insert_resource(physics::Seleced(None));
        app.add_system(physics::click_check);
        app.add_system(physics::item_hit);
        app.add_system(pickup::move_pickup_item);
        app.add_system(pickup::set_selected);
    }
}

#[derive(Clone, Copy, Component, Hash, PartialEq, Eq)]
pub struct ItemID(u64);

#[derive(Debug, Clone)]
struct Item<'a> {
    name: &'a str,
    icon: Handle<Image>,
}

pub struct ItemData {
    pub name: String,
    pub icon: Handle<Image>,
}

fn spawn_item(
    mut commands: Commands,
    icons: Res<crate::ui::UiIcons>,
    items: Res<Items>,
    mut set: ParamSet<(EventReader<ItemEvent>, EventWriter<ItemEvent>)>,
    item_settings: Res<config::ItemConfig>,
    window: Res<WindowDescriptor>,
){
    let mut send = Vec::new();
    for event in set.p0().iter() {
        match event {
            ItemEvent::Spawn(id) => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let width = window.width / 2.;
                let height = window.height / 2.;
                let x = rng.gen_range(-width..width);
                let y = rng.gen_range(-height..height);
                let s_id = commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {custom_size: Some(item_settings.frame_size), ..Default::default()},
                        texture: icons.get("item_frame"),
                        transform: Transform::from_translation(Vec3{x, y, z: 0.0}),
                        ..Default::default()
                    }
                )
                .with_children(|p| {
                    p.spawn_bundle(SpriteBundle{
                        sprite: Sprite {custom_size: Some(item_settings.icon_size), ..Default::default()},
                        texture: items.get(id).icon.clone(),
                        ..Default::default()
                    });
                })
                .insert(*id)
                .insert(physics::Size(item_settings.frame_size))
                .id();
                send.push(s_id);
            },
            ItemEvent::SpawnAt(id, loc) => {
                let s_id = commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {custom_size: Some(item_settings.frame_size), ..Default::default()},
                        texture: icons.get("item_frame"),
                        transform: Transform::from_translation(*loc),
                        ..Default::default()
                    }
                )
                .with_children(|p| {
                    p.spawn_bundle(SpriteBundle{
                        sprite: Sprite {custom_size: Some(item_settings.icon_size), ..Default::default()},
                        texture: items.get(&id).icon.clone(),
                        ..Default::default()
                    });
                })
                .insert(*id)
                .id();
                send.push(s_id);
            },
            _ => {},
        }
    }
    for id in send {
        set.p1().send(ItemEvent::Spawned(id));
    }
}

impl<'a> From<&'a ItemData> for Item<'a> {
    fn from(f: &'a ItemData) -> Self {
        Item { name: &f.name, icon: f.icon.clone() }
    }
}

impl From<&str> for ItemID {
    fn from(data: &str) -> Self {
        use std::hash::*;
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        data.hash(&mut hasher);
        ItemID(hasher.finish())
    }
}