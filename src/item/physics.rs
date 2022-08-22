use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, Component, Deref, DerefMut, PartialEq, Clone, Copy)]
pub struct Size(pub Vec2);

pub fn click_check(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Size), With<ItemID>>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCam>>,
    input: Res<Input<MouseButton>>,
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
                println!("hit entity: {:?}", hits[0].0);
                commands.insert_resource(Seleced(Some(hits[0].0)));
            }
        }
        if input.just_pressed(MouseButton::Right) {
            commands.insert_resource(Seleced(None));
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

pub struct Seleced(Option<Entity>);