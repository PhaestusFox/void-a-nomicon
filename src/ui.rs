use std::collections::HashMap;

use bevy::input::mouse::MouseWheel;

use crate::prelude::*;

pub mod ui_config;
mod tooltip;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TTTextStyle>();
        app.init_resource::<UiIcons>();
        app.init_resource::<ui_config::UiConfig>();
        app.add_plugin(tooltip::ToolTipPlugin);
        app.add_startup_system(load_ui_icons);
        app.add_startup_system(spawn_tool_tip);
        app.add_system(spawn_item_space_items);
        app.add_system(click_item);
        app.add_system(move_window);
    }
}


#[derive(Default)]
pub struct UiIcons{ 
    icons: HashMap<String, Handle<Image>>,
    default: Handle<Image>,
}

impl UiIcons {
    pub fn get(&self, name: &str) -> Handle<Image> {
        if let Some(h) = self.icons.get(name) {
            h.clone()
        } else {
            self.default.clone()
        }
    }
    pub fn load(&mut self, name: &str, path: &str, asset_server: &AssetServer) {
        self.icons.insert(name.to_string(), asset_server.load(path));
    }
}

fn load_ui_icons(
    asset_server: Res<AssetServer>,
    mut icons: ResMut<UiIcons>,
) {
    use std::fs;
    let ui_config = fs::read_to_string("./assets/ui_icons.config").unwrap();
    for line in ui_config.split('\n') {
        let mut line = line.split(':');
        let name = line.next();
        let path = line.next();
        match (name, path) {
            (Some(n), None) => warn!("Ui:{} has no path", n), 
            (Some(n), Some(p)) => icons.load(n.trim(), p.trim(), &asset_server),
            (None, Some(_)) => unreachable!("split should not return None then Some"),
            (None, None) => {},
        }
    }
}

struct TTTextStyle{
    tital: TextStyle,
    description: TextStyle,
}

impl FromWorld for TTTextStyle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load("Font.ttf");
        TTTextStyle{
            tital: TextStyle {
            font: font.clone(),
            font_size: 25.,
            color: Color::BLACK,
        },
        description: TextStyle {
            font,
            font_size: 12.5,
            color: Color::BLACK,
        },
        }
    }
}

fn spawn_tool_tip(
    mut commands: Commands,
    res: Res<UiIcons>,
    ttt: Res<TTTextStyle>,
) {
    use bevy::prelude::Size;
    commands.spawn_bundle(NodeBundle{
        style: Style {
            size: Size { width: Val::Percent(100.), height: Val::Px(150.) },
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },
        color: Color::BEIGE.into(),
        ..Default::default()
    })
    .with_children(|p| {
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(1260.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(130.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(NodeBundle {
            style: Style {
                size: Size { width: Val::Px(1260.), height: Val::Px(130.) },
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::BEIGE.into(),
            ..Default::default()
        })
        .insert(Name::new("Content"))
        .with_children(|p| {
            p.spawn_bundle(TextBundle{
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(0.), right: Val::Auto, top: Val::Px(0.), bottom: Val::Auto },
                    ..Default::default()
                },
                text: Text { sections: vec![
                    TextSection {style: ttt.tital.clone(),
                    value: "Name: ".to_string()},
                    TextSection {style: ttt.tital.clone(),
                        value: "{name here}".to_string(),
                    }], alignment: TextAlignment::default() },
                ..Default::default()
            }).insert(tooltip::ToolTipText::Name);
            p.spawn_bundle(TextBundle{
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(0.), right: Val::Auto, top: Val::Auto, bottom: Val::Px(0.) },
                    ..Default::default()
                },
                text: Text { sections: vec![TextSection {style: ttt.description.clone(), value: "{Description}".to_string()}], alignment: TextAlignment::default() },
                ..Default::default()
            }).insert(tooltip::ToolTipText::Description);
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(130.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(1260.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: res.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
    });
}

pub fn spawn_item_space(
    commands: &mut Commands,
    items: &Items,
    icons: &UiIcons,
    ui: &UiConfig,
) {
    use bevy::prelude::Size;
    let c = commands.spawn_bundle(NodeBundle{
        style: Style {
            size: Size { width: Val::Px(220.), height: Val::Px(570.) },
            flex_wrap: FlexWrap::Wrap,
            position_type: PositionType::Absolute,
            position: UiRect{top: Val::Px(0.), right: Val::Px(0.), left: Val::Auto, bottom: Val::Auto},
            ..Default::default()
        },
        color: Color::BEIGE.into(),
        ..Default::default()
    })
    .add_children(|p| {
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(200.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(650.) },
                ..Default::default()
            },
            ..Default::default()
        });
        let c = p.spawn_bundle(NodeBundle {
            style: Style {
                size: Size { width: Val::Px(200.), height: Val::Px(650.) },
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::FlexEnd,
                ..Default::default()
            },
            color: Color::BEIGE.into(),
            ..Default::default()
        })
        .insert(Name::new("Content"))
        .insert(ItemSpace).id();
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(650.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_line").into(),
            style: Style {
                size: Size { width: Val::Px(200.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        p.spawn_bundle(ImageBundle {
            image: icons.get("tooltip_end").into(),
            style: Style {
                size: Size { width: Val::Px(10.), height: Val::Px(10.) },
                ..Default::default()
            },
            ..Default::default()
        });
        c
    });
    for item in items.found() {
        spawn_item_space_item(items, item, commands, c, ui, icons);
    }
}

#[derive(Component)]
struct ItemSpace;

#[derive(Component)]
pub struct ItemSpaceItem;

fn spawn_item_space_items(
    mut commands: Commands,
    mut events: EventReader<ItemEvent>,
    items: Res<Items>,
    query: Query<Entity, With<ItemSpace>>,
    icons: Res<UiIcons>,
    ui: Res<UiConfig>,
) {
    let item_space = if let Ok(e) = query.get_single() {e} else {return;};
    for event in events.iter() {
        if let ItemEvent::Found(id) = event {
            spawn_item_space_item(&items, *id, &mut commands, item_space, &ui, &icons);
        }
    }
}

fn spawn_item_space_item(
    items: &Items,
    id: ItemID,
    commands: &mut Commands,
    item_space: Entity,
    ui: &UiConfig,
    icons: &UiIcons,
) {
    let item = items.get(&id);
    if item.name() == "Debug Item" {return;}
    commands.entity(item_space).with_children(|p| {
        p.spawn_bundle(ButtonBundle{
            image: icons.get("item_frame").into(),
            style: ui.frame_style.clone(),
            ..Default::default()
        })
        .insert(ItemSpaceItem)
        .insert(id)
        .with_children(|p| {
            p.spawn_bundle(ImageBundle{
                image: item.icon().into(),
                style: ui.icon_style.clone(),
                focus_policy: bevy::ui::FocusPolicy::Pass,
                ..Default::default()
            });
        });
    });
}

fn click_item(
    query: Query<(&ItemID, &Interaction), (With<ItemSpaceItem>, Changed<Interaction>)>,
    mut events: EventWriter<ItemEvent>,
) {
    for (id, click) in query.iter() {
        match click {
            Interaction::Clicked => {
            events.send(ItemEvent::Spawn(*id));
            }
            Interaction::Hovered => {
                events.send(ItemEvent::ToolTip(*id));
            }
            _ => {}
        }
    }
}

fn move_window(
    mut events: EventReader<MouseWheel>,
    mut query: Query<&mut Style, With<ItemSpace>>,
) {
    let mut style = if let Ok(e) = query.get_single_mut() {e} else {return;};
    let mut delta = 0.0;
    for e in events.iter() {
        delta += e.y;
    }
    if delta != 0. {
        delta *= 10.;
        if let Val::Px(v) = style.position.bottom {style.position.bottom = Val::Px(v + delta);
        } else {
            style.position.bottom = Val::Px(delta);
        };

    }
}