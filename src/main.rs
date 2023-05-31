use bevy::{prelude::*, render::texture::ImageSettings};
use void_a_nomicon::prelude::*;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_editor_pls::prelude::EditorPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugin(void_a_nomicon::one_offs::OneOffPlugin)
        .add_plugin(void_a_nomicon::ui::UiPlugin)
        .add_plugin(void_a_nomicon::item::ItemPlugin)
        .add_plugin(void_a_nomicon::recipies::RecipiePlugin)
        .add_plugin(void_a_nomicon::serde::SaveLoadPlugin)
        .add_plugin(void_a_nomicon::story::StoryPlugin)
        .add_plugin(void_a_nomicon::sound::SoundPlugin)
        .insert_resource(WindowDescriptor{
            width: 1280.,
            height: 720.,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(setup)
        
        //.add_startup_system(load_test_item)
        .add_system(spawn_test_item)

        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default())
    .insert(MainCam);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        transform: Transform::from_translation(Vec3::Z * 10.),
        ..Default::default()
    })
    .insert(void_a_nomicon::one_offs::Splach(1., 0.69));
}

fn spawn_test_item(
   mut events: EventWriter<ItemEvent>,
   input: Res<Input<KeyCode>>,
){
    if input.just_pressed(KeyCode::B) {
        events.send(ItemEvent::Spawn(ItemID::from("Bevy")))
    }
}