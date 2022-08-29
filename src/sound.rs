use crate::prelude::*;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(load_test_sound)
        .add_system(test_sound);
    }
}

struct Pop(Handle<AudioSource>);

fn load_test_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Pop(asset_server.load("sounds/pop.wav")));
}
fn test_sound(
    input: Res<Input<KeyCode>>,
    pop: Res<Pop>,
    audio: Res<Audio>,
    mut events: EventReader<ItemEvent>,
    items: Res<Items>,
) {
    if input.just_pressed(KeyCode::P) {
        info!("playing sound");
        audio.play(pop.0.clone());
    }
    for event in events.iter() {
        match event {
            ItemEvent::Spawn(id) |
            ItemEvent::SpawnAt(id,_) => {
                audio.play(items.get(id).sound());
            },
            _ => {},
        }
    }
}