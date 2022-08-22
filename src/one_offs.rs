use bevy::prelude::*;

pub struct OneOffPlugin;

impl Plugin for OneOffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clear_splash);
    }
}

#[derive(Debug, Component)]
pub struct Splach(pub f32);

fn clear_splash(
    mut commands: Commands,
    mut splash: Query<(Entity, &mut Splach)>,
    time: Res<Time>,
) {
    for (e, mut splash) in splash.iter_mut() {
        splash.0 -= time.delta_seconds();
        if splash.0 < 0.0 {
            commands.entity(e).despawn_recursive();
        }
    }
}