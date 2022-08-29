use bevy::prelude::*;

pub struct UiConfig {
    pub frame_style: Style,
    pub icon_style: Style,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UiConfigData {
    frame_size: f32,
    icon_size: f32,
}

impl Default for UiConfigData {
    fn default() -> Self {
        UiConfigData { frame_size: 100., icon_size: 100. }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        let data = if let Ok(str) = std::fs::read_to_string("./assets/ui.config") {str} else {
            let f = UiConfigData::default();
            return UiConfig {
                frame_style: Style {
                    size: Size { width: Val::Px(f.frame_size), height: Val::Px(f.frame_size) },
                    ..Default::default()
                }, icon_style: Style {
                    size: Size { width: Val::Px(f.frame_size), height: Val::Px(f.frame_size) },
                    margin: UiRect::all(Val::Auto),
                    ..Default::default()
                },
            };
        };
        if let Ok(data) = ron::from_str::<UiConfigData>(&data) {
            data.into()
        } else {
            let f = UiConfigData::default();
            UiConfig {
                frame_style: Style {
                    size: Size { width: Val::Px(f.frame_size), height: Val::Px(f.frame_size) },
                    ..Default::default()
                }, icon_style: Style {
                    size: Size { width: Val::Px(f.frame_size), height: Val::Px(f.frame_size) },
                    margin: UiRect::all(Val::Auto),
                    ..Default::default()
                },
            }
        }
    }
}

impl From<UiConfigData> for UiConfig {
    fn from(f: UiConfigData) -> Self {
        UiConfig { frame_style: Style {
            size: Size { width: Val::Px(f.frame_size), height: Val::Px(f.frame_size) },
            ..Default::default()
        }, icon_style: Style {
            size: Size { width: Val::Px(f.icon_size), height: Val::Px(f.icon_size) },
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        }
    }
}
