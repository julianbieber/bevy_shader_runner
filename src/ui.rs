use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use bevy::{
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{
        Slider, SliderRange, SliderValue, UiWidgetsPlugins,
        ValueChange, observe,
    },
};
use serde::{Deserialize, Serialize};

use crate::{CustomMaterial, ShaderPath};

#[derive(Serialize, Deserialize, Resource, Debug)]
pub struct SliderState {
    pub sliders_1: Vec4,
    pub sliders_2: Vec4,
    pub sliders_3: Vec4,
    pub sliders_4: Vec4,
    pub sliders_5: Vec4,
}

impl Default for SliderState {
    fn default() -> Self {
        SliderState {
            sliders_1: Vec4::ZERO,
            sliders_2: Vec4::ZERO,
            sliders_3: Vec4::ZERO,
            sliders_4: Vec4::ZERO,
            sliders_5: Vec4::ZERO,
        }
    }
}

pub fn read_slider_state(shader_path: &Path) -> Option<SliderState> {
    let config_path = extract_config_path(shader_path)?;

    let json_confg = read_to_string(config_path).ok()?;
    let config: SliderState = serde_json::from_str(&json_confg).unwrap();

    Some(config)
}

pub fn read_slider_state_for_dump(shader_path: &Path) -> Option<SliderState> {
    let shader_name = {
        let shader_name = shader_path.file_name()?;
        let extension = shader_path.extension()?;
        shader_name
            .to_string_lossy()
            .replace(extension.to_string_lossy().as_ref(), "")
    };
    
    // For dump functionality, we want to look in assets/configs directly
    let config_path = PathBuf::from("assets").join("configs").join(format!("{}{}", shader_name, "json"));
    
    let json_confg = read_to_string(config_path).ok()?;
    let config: SliderState = serde_json::from_str(&json_confg).unwrap();

    Some(config)
}

fn write_slider_state(shader_path: &Path, config: &SliderState) -> Option<()> {
    let json_confg = serde_json::to_string(config).ok()?;

    let config_path = extract_config_path(shader_path)?;

    std::fs::write(config_path, json_confg).unwrap();

    Some(())
}

fn extract_config_path(shader_path: &Path) -> Option<PathBuf> {
    let shader_name = {
        let shader_name = shader_path.file_name()?;
        let extension = shader_path.extension()?;
        shader_name
            .to_string_lossy()
            .replace(extension.to_string_lossy().as_ref(), "")
    };
    let base_dir = shader_path.parent()?.parent()?;
    let assets = PathBuf::from("assets");
    let config_path = assets.join(base_dir.join("configs").join(format!("{}{}", shader_name, "json")));
    Some(config_path)
}

impl From<SliderState> for CustomMaterial {
    fn from(value: SliderState) -> Self {
        CustomMaterial {
            time: 0.0,
            resolution: Vec2::ZERO,
            sliders_1: value.sliders_1,
            sliders_2: value.sliders_2,
            sliders_3: value.sliders_3,
            sliders_4: value.sliders_4,
            sliders_5: value.sliders_5,
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiWidgetsPlugins);

        app.add_systems(Startup, setup_ui)
            .add_systems(Update, show_hide_ui);
    }
}

#[derive(Component)]
struct SliderMarker(u32);
#[derive(Component)]
struct UIRootMarker;

fn setup_ui(mut commands: Commands, initial_state: Res<SliderState>) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            min_height: px(10.),
            width: Val::Percent(100.0),
            border: UiRect::all(px(1)),
            position_type: PositionType::Absolute,
            ..default()
        },
        Visibility::Visible,
        UIRootMarker,
        children![
            create_slider_block(0, initial_state.sliders_1),
            create_slider_block(4, initial_state.sliders_2),
            create_slider_block(8, initial_state.sliders_3),
            create_slider_block(12, initial_state.sliders_4),
            create_slider_block(16, initial_state.sliders_5),
        ],
    ));
}

fn show_hide_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut ui_root: Single<&mut Visibility, With<UIRootMarker>>,
) {
    if keys.just_pressed(KeyCode::KeyM) {
        ui_root.toggle_visible_hidden();
    }
}

fn on_update_slider(
    value_change: On<ValueChange<f32>>,
    sliders: Query<&SliderMarker>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    handle: Single<&MeshMaterial2d<CustomMaterial>>,
    mut commands: Commands,
    mut slider_state: ResMut<SliderState>,
    shader_path: Res<ShaderPath>,
) {
    let slider_entity = value_change.event().source;
    commands
        .entity(slider_entity)
        .insert(SliderValue(value_change.value));
    let slider = sliders.get(slider_entity).unwrap();
    if let Some(m) = materials.get_mut(handle.id()) {
        match slider.0 {
            0 => m.sliders_1.x = value_change.value,
            1 => m.sliders_1.y = value_change.value,
            2 => m.sliders_1.z = value_change.value,
            3 => m.sliders_1.w = value_change.value,
            4 => m.sliders_2.x = value_change.value,
            5 => m.sliders_2.y = value_change.value,
            6 => m.sliders_2.z = value_change.value,
            7 => m.sliders_2.w = value_change.value,
            8 => m.sliders_3.x = value_change.value,
            9 => m.sliders_3.y = value_change.value,
            10 => m.sliders_3.z = value_change.value,
            11 => m.sliders_3.w = value_change.value,
            12 => m.sliders_4.x = value_change.value,
            13 => m.sliders_4.y = value_change.value,
            14 => m.sliders_4.z = value_change.value,
            15 => m.sliders_4.w = value_change.value,
            16 => m.sliders_5.x = value_change.value,
            17 => m.sliders_5.y = value_change.value,
            18 => m.sliders_5.z = value_change.value,
            19 => m.sliders_5.w = value_change.value,
            _ => (),
        }
        slider_state.sliders_1 = m.sliders_1;
        slider_state.sliders_2 = m.sliders_2;
        slider_state.sliders_3 = m.sliders_3;
        slider_state.sliders_4 = m.sliders_4;
        slider_state.sliders_5 = m.sliders_5;
        write_slider_state(&shader_path.0, &slider_state);
    }
}





fn create_slider_block(start: u32, values: Vec4) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12 * 4),
            width: percent(100),
            ..default()
        },
        children![
            (create_slider(start, values.x), observe(on_update_slider)),
            (
                create_slider(start + 1, values.y),
                observe(on_update_slider)
            ),
            (
                create_slider(start + 2, values.z),
                observe(on_update_slider)
            ),
            (
                create_slider(start + 3, values.w),
                observe(on_update_slider)
            ),
        ],
    )
}

fn create_slider(index: u32, initial: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            ..default()
        },
        TabIndex(index as i32),
        SliderValue(initial),
        SliderMarker(index),
        SliderRange::new(0.0, 1.0),
        Slider {
            track_click: bevy::ui_widgets::TrackClick::Snap,
        },
        Hovered::default(),
    )
}
