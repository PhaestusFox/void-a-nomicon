use crate::{prelude::*, item::Items};

pub struct ToolTipPlugin;
impl Plugin for ToolTipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_tooltip);
        app.add_system(update_tooltip);
    }
}

#[derive(Debug, Component)]
pub enum ToolTipText {
    Name,
    Description,
}

fn update_tooltip(
    mut text: Query<(&mut Text, &ToolTipText)>,
    items: Res<Items>,
    mut events: EventReader<ItemEvent>,
) {
    for event in events.iter() {
        if let ItemEvent::ToolTip(item) = event {
            let item = items.get(item);
            for (mut text, id) in text.iter_mut() {
                match id {
                    ToolTipText::Name => {
                        text.sections[1].value = item.name().to_string();
                    },
                    ToolTipText::Description => {
                        text.sections[0].value = item.description().to_string();
                    },
                }
            }
        }
    }
}

pub fn set_tooltip(
    query: Query<(&Transform, &Size, &ItemID)>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::MainCam>>,
    input: Res<Input<MouseButton>>,
    mut events: EventWriter<ItemEvent>
) {
    let (camera, global_transform) = camera.single();
    let window = windows.get_primary().unwrap();
    if let Some(screen_pos) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world = global_transform.compute_matrix() * camera.projection_matrix().inverse();

        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let world_pos: Vec2 = world_pos.truncate();

        //if input.just_pressed(MouseButton::Right) {
            let mut hits: Vec<(ItemID, f32)> = Vec::new();
            for (transform, size, item) in query.iter() {
                if crate::item::physics::box_point_hit(size.0, transform.translation.truncate(), world_pos) {
                    hits.push((*item, transform.translation.z));
                }
            }
            if hits.len() != 0 {
                hits.sort_by(|(_, a),(_, b)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Greater));
                events.send(ItemEvent::ToolTip(hits[0].0));
            }
        //}
        // } else if input.just_pressed(MouseButton::Right) {
        //     let mut hits: Vec<(Entity, f32)> = Vec::new();
        //     for (e, transform, size) in query.iter() {
        //         if box_point_hit(size.0, transform.translation.truncate(), world_pos) {
        //             hits.push((e, transform.translation.z));
        //         }
        //     }
        //     if hits.len() != 0 {
        //         hits.sort_by(|(_, a),(_, b)| b.partial_cmp(a).unwrap_or(Ordering::Greater));
        //         events.send(ItemEvent::ToolTip(hits[0].0));
        //     }
        // }
    }
}