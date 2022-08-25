use bevy::prelude::*;

mod prelude;
mod one_offs;
mod error;
mod ui;
mod item;
mod recipies;
mod serde;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_editor_pls::prelude::EditorPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugin(one_offs::OneOffPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(item::ItemPlugin)
        .add_plugin(recipies::RecipiePlugin)
        .add_plugin(serde::SaveLoadPlugin)
        .insert_resource(WindowDescriptor{
            width: 1280.,
            height: 720.,
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
}

#[derive(Component)]
pub struct MainCam;

fn spawn_test_item(
   mut events: EventWriter<item::ItemEvent>,
   input: Res<Input<KeyCode>>,
){
    if input.just_pressed(KeyCode::Space) {
        events.send(item::ItemEvent::Spawn(item::ItemID::from("Test Item")))
    } else if input.just_pressed(KeyCode::B) {
        events.send(item::ItemEvent::Spawn(item::ItemID::from("Bevy")))
    }
}