use bevy::ecs::schedule::ShouldRun;

use crate::{prelude::*, ui::UiIcons};

const SAVE_PATH: &'static str = "./assets/story.sav";

enum StoryEvent {
    SpawnItemSpace,
}

pub struct BevyCount(pub usize);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StoryFlags {
    #[serde(default)]
    unlocked_app: bool,
    #[serde(default)]
    has_trash: bool,
    #[serde(default)]
    has_items_space: bool,
}

fn save_flag(flags: &StoryFlags) {
    match std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(SAVE_PATH) {
    Ok(mut f) => {
        if let Err(e) = ron::ser::to_writer_pretty(&mut f, flags, ron::ser::PrettyConfig::default()) {
            error!("{}",e);
        }
    },
    Err(e) => {error!("{}", e);}
    }
}

impl Default for StoryFlags {
    fn default() -> Self {
        if let Ok(str) =  std::fs::read_to_string(SAVE_PATH) {
            if let Ok(save) = ron::from_str(&str) {
                return save;
            }
        }
        StoryFlags { unlocked_app: false, has_trash: false, has_items_space: false }
    }
}

pub struct StoryPlugin;
impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StoryFlags>();
        app.insert_resource(BevyCount(0));
        app.add_system_to_stage(CoreStage::First,count_bevys);
        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(spawn_voids)
            .with_system(spawn_void)
        );
        app.add_system(update_flags);
        app.add_system_set(
            SystemSet::new()
            .with_run_criteria(can_spawn_time)
            .with_system(spawn_time)
        );
        app.add_startup_system(load_story);
        app.add_event::<StoryEvent>();
        app.add_system(story_events);
    }
}

fn load_story(
    res: Res<StoryFlags>,
    mut events: EventWriter<ItemEvent>,
    mut story_events: EventWriter<StoryEvent>,
) {
    if res.has_trash {
        events.send(ItemEvent::SpawnAt(ItemID::new("Trash"), Vec3::new(-590., 310., 0.0)));
    }
    if res.unlocked_app {
        events.send(ItemEvent::Spawn(ItemID::new("Totally a game")));
    }
    if res.has_items_space {
        story_events.send(StoryEvent::SpawnItemSpace);
    }
}

fn story_events(
    mut events: EventReader<StoryEvent>,
    ui: Res<UiConfig>,
    icons: Res<UiIcons>,
    items: Res<Items>,
    mut commands: Commands,
) {
    for event in events.iter() {
        match event {
            StoryEvent::SpawnItemSpace => {crate::ui::spawn_item_space(&mut commands, &items, &icons, &ui);}
        }
    }
}

fn count_bevys(
    mut res: ResMut<BevyCount>,
    mut events: EventReader<ItemEvent>,
) {
    for event in events.iter() {
        match event {
            ItemEvent::Spawn(id) |
            ItemEvent::SpawnAt(id,_) => {
                if id == &ItemID::from("Bevy") {
                    res.0 += 1;
                }
            },
            _ => {},
        }
    }
}

fn update_flags(
    mut res: ResMut<StoryFlags>,
    mut events: EventReader<ItemEvent>,
    mut story_events: EventWriter<StoryEvent>,
) {
    let app_id = ItemID::new("Totally a game");
    let trash = ItemID::new("Trash");
    let item_box = ItemID::new("Item Space");
    let mut save = false;
    for event in events.iter() {
        match event {
            ItemEvent::Spawn(id) |
            ItemEvent::SpawnAt(id, _) => {
                if !res.unlocked_app && id.id() == app_id.id() {
                    res.unlocked_app = true;
                    save = true;
                }
                if !res.has_trash && id.id() == trash.id() {
                    res.has_trash = true;
                    save = true;
                }
                if !res.has_items_space && id.id() == item_box.id() {
                    res.has_items_space = true;
                    save = true;
                    story_events.send(StoryEvent::SpawnItemSpace);
                }
            },
            _ => {}
        }
    }
    if save {
        save_flag(&res);
    }
}

fn spawn_void(
    query: Query<&ItemID>,
    mut events: EventWriter<ItemEvent>,
) {
    let mut voids = 0;
    let void_id = ItemID::from("Void");
    for id in query.iter() {
        if id == &void_id {voids += 1;}
    }
    if voids < 5 {
        events.send(ItemEvent::Spawn(void_id));
    }
}

fn spawn_voids(
    res: Res<StoryFlags>,
    mut local: Local<f32>,
    time: Res<Time>,
) -> ShouldRun {
    *local += time.delta_seconds();
    if *local < 10. {
        return ShouldRun::No
    }
    *local %= 10.;
    if res.unlocked_app {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn spawn_time(
    mut events: EventWriter<ItemEvent>,
) {
    events.send(ItemEvent::Spawn(ItemID::new("Time")));
}

fn can_spawn_time(
    res: Res<StoryFlags>,
    mut local: Local<f32>,
    time: Res<Time>,
) -> ShouldRun {
    *local += time.delta_seconds();
    if *local < 150. {
        return ShouldRun::No
    }
    *local %= 150.;
    if res.unlocked_app {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}