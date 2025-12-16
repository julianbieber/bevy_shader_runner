use std::sync::OnceLock;

use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite_render::{Material2d, Material2dPlugin},
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
            Material2dPlugin::<CustomMaterial>::default(),
            ShaderViewerPlugin {},
        ))
        .run()
}

struct ShaderViewerPlugin {}

impl Plugin for ShaderViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, update_time);
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
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(CustomMaterial {
            time: 0.0,
            resolution: window.size(),
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
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

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    resolution: Vec2,
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
