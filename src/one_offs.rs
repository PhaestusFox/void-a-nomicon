use crate::{prelude::*, story::BevyCount};

pub struct OneOffPlugin;

impl Plugin for OneOffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(clear_splash);
    }
}

#[derive(Debug, Component)]
pub struct Splach(pub f32, pub f32);

fn clear_splash(
    mut commands: Commands,
    mut splash: Query<(Entity, &mut Splach, &mut Transform)>,
    time: Res<Time>,
    mut events: EventWriter<ItemEvent>,
    bevys: Res<BevyCount>,
) {
    for (e, mut splash, mut t) in splash.iter_mut() {
        if splash.0 < 0.0 {
            if splash.1 < 0.0 {
                commands.entity(e).despawn_recursive();
                if bevys.0 < 2 {
                    events.send(ItemEvent::SpawnAt(ItemID::from("Bevy"), Vec3::ZERO));
                }
            } else {
                t.scale = Vec3::splat(splash.1 + 0.31);
                splash.1 -= time.delta_seconds() * 2.0;
            }
        } else {
            splash.0 -= time.delta_seconds();
        }
    }
}