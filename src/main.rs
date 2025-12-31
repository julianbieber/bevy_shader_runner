use std::{path::PathBuf, sync::OnceLock};
mod ui;

use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use clap::Parser;

use crate::ui::{SliderState, UiPlugin, read_slider_state};

#[derive(Parser)]
struct Opt {
    #[arg(long)]
    shader: String,
}
static FRAGMENT: OnceLock<String> = OnceLock::new();

#[derive(Resource)]
pub struct ShaderPath(PathBuf);

fn main() -> AppExit {
    let opt = Opt::parse();
    let shader_path = PathBuf::from(&opt.shader);
    let config = read_slider_state(&shader_path).unwrap_or_default();
    FRAGMENT.set(opt.shader).unwrap();
    App::new()
        .insert_resource(config)
        .insert_resource(ShaderPath(shader_path))
        .add_plugins((
            DefaultPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
            Material2dPlugin::<CustomMaterial>::default(),
            UiPlugin,
            ShaderViewerPlugin {},
        ))
        .run()
}

struct ShaderViewerPlugin {}

impl Plugin for ShaderViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera,))
            .add_systems(Update, (update_time, react_to_resize));
    }
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window: Single<&Window>,
    initial_state: Res<SliderState>,
) {
    commands.spawn(Camera2d {});
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(window.width(), window.height()))),
        MeshMaterial2d(materials.add(CustomMaterial {
            time: 0.0,
            resolution: window.size(),
            sliders_1: initial_state.sliders_1,
            sliders_2: initial_state.sliders_2,
            sliders_3: initial_state.sliders_3,
            sliders_4: initial_state.sliders_4,
            sliders_5: initial_state.sliders_5,
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
    #[uniform(3)]
    sliders_2: Vec4,
    #[uniform(4)]
    sliders_3: Vec4,
    #[uniform(5)]
    sliders_4: Vec4,
    #[uniform(6)]
    sliders_5: Vec4,
}

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
