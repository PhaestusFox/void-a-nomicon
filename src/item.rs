use crate::prelude::*;

mod config;
mod items;
mod event;
pub mod physics;
mod pickup;

pub mod tags;

pub use items::Items;
pub use event::ItemEvent;

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemEvent>();
        app.init_resource::<Items>();
        app.init_resource::<config::ItemConfig>();
        app.add_system(spawn_item);
        app.add_system_to_stage(CoreStage::PostUpdate, event::move_down);
        app.insert_resource(physics::Seleced(None));
        app.add_system(physics::click_check);
        app.add_system(physics::detect_drop);
        app.add_system(physics::item_hit);
        app.add_system(pickup::move_pickup_item);
        app.add_system(pickup::set_selected);
        app.add_system(items::found_update);
    }
}

#[derive(Debug, Clone, Copy, Component, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemID(u64);

impl ItemID {
    pub fn new<T>(name: T) -> ItemID where T: Into<ItemID> {
        name.into()
    }

    #[inline]
    pub fn first(self, other: Self) -> (ItemID, ItemID) {
        if self.0 < other.0 {
            (self, other)
        } else {
            (other, self)
        }
    }

    #[inline(always)]
    pub fn id(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Item<'a> {
    name: &'a str,
    icon: &'a Handle<Image>,
    description: &'a str,
    sound: &'a Handle<AudioSource>,
}

impl Item<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn icon(&self) -> Handle<Image> {
        self.icon.clone()
    }
    pub fn description(&self) -> &str {
        self.description
    }
    pub fn sound(&self) -> Handle<AudioSource> {
        self.sound.clone()
    }
}

pub struct ItemData {
    pub name: String,
    pub description: String, 
    pub icon: Handle<Image>,
    pub tags: tags::Tags,
    pub sound: Handle<AudioSource>,
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
                let x = rng.gen_range(-width..(width - 200.));
                let y = rng.gen_range((-height + 150.0)..height);
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
                        transform: Transform::from_translation(Vec3::Z * 0.1),
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
                        transform: Transform::from_translation(Vec3::Z * 0.1),
                        ..Default::default()
                    });
                })
                .insert(*id)
                .insert(physics::Size(item_settings.frame_size))
                .id();
                send.push(s_id);
            },
            _ => {},
        }
    }
    for id in send {
        set.p1().send(ItemEvent::Spawned(id));
        println!("send spawned {:?}", id);
    }
}

impl<'a> From<&'a ItemData> for Item<'a> {
    fn from(f: &'a ItemData) -> Self {
        Item { name: &f.name, icon: &f.icon, description: &f.description, sound: &f.sound }
    }
}

impl From<&str> for ItemID {
    fn from(data: &str) -> Self {
        use std::hash::*;
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        let data = data.replace(' ', "_").to_lowercase();
        data.hash(&mut hasher);
        ItemID(hasher.finish())
    }
}

impl From<String> for ItemID {
    fn from(data: String) -> Self {
        use std::hash::*;
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        let data = data.replace(' ', "_").to_lowercase();
        data.hash(&mut hasher);
        ItemID(hasher.finish())
    }
}