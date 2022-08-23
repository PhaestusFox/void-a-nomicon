use bevy::input::mouse::MouseMotion;

pub use crate::prelude::*;

use super::physics::Seleced;

pub fn move_pickup_item(
    res: Res<super::physics::Seleced>,
    mut transforms: Query<&mut Transform, With<ItemID>>,
    mut events: EventReader<MouseMotion>,
) {
    let e = if let Some(e) = res.0 {e} else {return;};
    let mut transform = if let Ok(t) =  transforms.get_mut(e) {t} else {return;};
    let mut delta = Vec2::ZERO;
    for event in events.iter() {
        delta += event.delta;
    }
    if !delta.is_finite() {return;}
    transform.translation.x += delta.x;
    transform.translation.y -= delta.y;
}

pub fn set_selected(
    mut commands: Commands,
    mut set: ParamSet<(EventReader<ItemEvent>, EventWriter<ItemEvent>)>,
    res: Res<Seleced>,
) {
    let mut send = None;
    for event in set.p0().iter() {
        match event {
            ItemEvent::Pickup(e) => {commands.insert_resource(Seleced(Some(*e)))},
            ItemEvent::Drop => {
                if let Some(e) = res.0 {
                    send = Some(ItemEvent::Droped(e));
                    commands.insert_resource(Seleced(None));
                } else {
                    warn!("Called drop when nothing was selected");
                }
            },
            _ => {},
        }
    }
    if let Some(send) = send {
        set.p1().send(send);
    }
}