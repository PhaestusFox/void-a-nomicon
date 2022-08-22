use crate::prelude::*;

pub enum ItemEvent {
    Spawn(ItemID),
    SpawnAt(ItemID, Vec3),
}

pub fn move_down(
   mut h: Local<f32>,
   mut query: Query<&mut Transform, Added<ItemID>>
) {
    for mut trans in query.iter_mut() {
        *h += 0.001;
        trans.translation.z = *h;
    }
}