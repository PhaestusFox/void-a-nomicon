use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, PartialEq, Clone, Copy)]
pub struct Size(pub Vec2);

pub fn click_check(
    query: Query<(Entity, &Transform, &Size), With<ItemID>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCam>>,
    input: Res<Input<MouseButton>>,
    mut events: EventWriter<ItemEvent>,
){
    let (camera, global_transform) = camera.single();

    let window = windows.get_primary().unwrap();

    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = global_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();

        if input.just_pressed(MouseButton::Left) {
            let mut hits: Vec<(Entity, f32)> = Vec::new();
            for (e, transform, size) in query.iter() {
                // println!("Click At {}:{}", world_pos.x, world_pos.y);
                // println!("Entity At {}:{}", transform.translation.x, transform.translation.y);
                if box_point_hit(size.0, transform.translation.truncate(), world_pos) {
                    hits.push((e, transform.translation.z));
                }
            }
            if hits.len() != 0 {
                hits.sort_by(|(_, a),(_, b)| b.partial_cmp(a).unwrap_or(Ordering::Greater));
                debug!("hit entity: {:?}", hits[0].0);
                events.send(ItemEvent::Pickup(hits[0].0));
            }
        }
        if input.just_released(MouseButton::Left) {
            events.send(ItemEvent::Drop);
        }
    }
}

fn box_point_hit(
    size: Vec2,
    center: Vec2,
    point: Vec2,
) -> bool {
    let x_off = size.x / 2.0;
    let y_off = size.y / 2.0;
    point.x < center.x + x_off && point.x > center.x - x_off && point.y < center.y + y_off && point.y > center.y - y_off
}

pub fn box_box_hit(
    size0: Vec2,
    center0: Vec2,
    size1: Vec2,
    center1: Vec2,
) -> bool {
    let x0_off = size0.x / 2.0;
    let x1_off = size1.x / 2.0;
    let y0_off = size0.y / 2.0;
    let y1_off = size1.y / 2.0;
    
    center0.x + x0_off > center1.x - x1_off && center0.x - x0_off < center1.x + x1_off &&
    center0.y + y0_off > center1.y - y1_off && center0.y - y0_off < center1.y + y1_off
}

pub fn box_box_overlap(
    size0: Vec2,
    center0: Vec2,
    size1: Vec2,
    center1: Vec2,
) -> f32 {
    let x0 = (center0.x - (size0.x / 2.0)).max(center1.x - (size1.x/2.0));
    let x1 = (center0.x + (size0.x / 2.0)).min(center1.x + (size1.x/2.0));
    let y0 = (center0.y - (size0.y / 2.0)).max(center1.y - (size1.y/2.0));
    let y1 = (center0.y + (size0.y / 2.0)).min(center1.y + (size1.y/2.0));
    let x = (x0 - x1).abs();
    let y = (y0 - y1).abs();
    x * y
}

#[derive(Debug, Deref)]
pub struct Seleced(pub Option<Entity>);