use crate::prelude::*;

pub enum ItemEvent {
    ToolTip(ItemID),
    Spawn(ItemID),
    SpawnAt(ItemID, Vec3),
    Spawned(Entity),
    Pickup(Entity),
    Drop,
    Droped(Entity),
    CheckCombine(Entity, Entity),
    Found(ItemID),
}

pub fn move_down(
   mut h: Local<f32>,
   mut query: Query<&mut Transform>,
   mut events: EventReader<ItemEvent>,
) {
    for event in events.iter() {
        match event {
            ItemEvent::Pickup(e) |
            ItemEvent::Spawned(e) => {
                if let Ok(mut t) = query.get_mut(*e) {
                    *h += 0.001;
                    t.translation.z = *h;
                } else {
                    warn!("failed to find entity: {:?}", e);
                }
            },
            _ => {},
        }
    }
}