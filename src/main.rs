use std::sync::OnceLock;

use bevy::{
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabIndex, TabNavigationPlugin},
    },
    picking::hover::Hovered,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
    ui_widgets::{
        CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, UiWidgetsPlugins,
        ValueChange, observe,
    },
    window::WindowResized,
};
use clap::Parser;

#[derive(Parser)]
struct Opt {
    #[arg(long)]
    shader: String,
}
static FRAGMENT: OnceLock<String> = OnceLock::new();

fn main() -> AppExit {
    let opt = Opt::parse();
    FRAGMENT.set(opt.shader).unwrap();
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
            Material2dPlugin::<CustomMaterial>::default(),
            ShaderViewerPlugin {},
        ))
        .run()
}

struct ShaderViewerPlugin {}

impl Plugin for ShaderViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_ui))
            .add_systems(
                Update,
                (
                    update_time,
                    react_to_resize,
                    update_slider_style,
                    show_hide_ui,
                ),
            );
    }
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window: Single<&Window>,
) {
    commands.spawn(Camera2d {});
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(window.width(), window.height()))),
        MeshMaterial2d(materials.add(CustomMaterial {
            time: 0.0,
            resolution: window.size(),
            sliders_1: Vec4::ZERO,
        })),
    ));
}

fn update_time(
    mut materials: ResMut<Assets<CustomMaterial>>,
    handles: Query<&MeshMaterial2d<CustomMaterial>>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    for handle in handles {
        if let Some(m) = materials.get_mut(handle.id()) {
            m.time = time.elapsed_secs();
            m.resolution = window.size()
        }
    }
}

fn react_to_resize(
    mut resized: MessageReader<WindowResized>,
    window: Single<&Window>,
    mut rect: Single<&mut Mesh2d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if resized.read().next().is_some() {
        rect.0 = meshes.add(Rectangle::new(window.width(), window.height()));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    resolution: Vec2,
    #[uniform(2)]
    sliders_1: Vec4,
}

#[derive(Component)]
struct SliderMarker(u32);

impl Material2d for CustomMaterial {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        if FRAGMENT.get().unwrap().ends_with(".frag") {
            "shaders/default.vert".into()
        } else {
            "shaders/default.wgsl".into()
        }
    }

    fn fragment_shader() -> bevy::shader::ShaderRef {
        FRAGMENT.get().unwrap().as_str().into()
    }
}

#[derive(Component)]
struct UIRootMarker;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            min_height: px(10.),
            min_width: Val::Percent(20.),
            border: UiRect::all(px(1)),
            position_type: PositionType::Absolute,
            ..default()
        },
        Visibility::Visible,
        UIRootMarker,
        children![
            (create_slider(0), observe(on_update_slider)),
            (create_slider(1), observe(on_update_slider)),
            (create_slider(2), observe(on_update_slider)),
            (create_slider(3), observe(on_update_slider)),
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
            _ => (),
        }
        dbg!(m.sliders_1);
    }
}

fn update_slider_style(
    sliders: Query<
        (Entity, &SliderValue, &SliderRange),
        (
            Or<(
                Changed<SliderValue>,
                Changed<SliderRange>,
                Changed<Hovered>,
                Changed<CoreSliderDragState>,
            )>,
        ),
    >,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
    for (slider_ent, value, range) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok(mut thumb_node) = thumbs.get_mut(child) {
                thumb_node.left = percent(range.thumb_position(value.0 * 100.0));
            }
        }
    }
}

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

fn create_slider(index: u32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: px(300),
            ..default()
        },
        TabIndex(index as i32),
        SliderValue(0.0),
        SliderMarker(index),
        SliderRange::new(0.0, 1.0),
        Slider {
            track_click: bevy::ui_widgets::TrackClick::Snap,
        },
        Hovered::default(),
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: px(6),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK), // Border color for the slider
                BorderRadius::all(px(3)),
            )),
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 12px to accommodate the thumb.
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    // Thumb
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0), // This will be updated by the slider's value
                        ..default()
                    },
                    BorderRadius::MAX,
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}
