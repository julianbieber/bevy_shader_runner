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
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2d {
        ..Default::default()
    });
    commands.spawn((
        Mesh2d(meshes.add(Cuboid::default())),
        MeshMaterial2d(materials.add(CustomMaterial { time: 0.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> bevy::shader::ShaderRef {
        "shaders/default.vert".into()
    }

    fn fragment_shader() -> bevy::shader::ShaderRef {
        FRAGMENT.get().unwrap().as_str().into()
    }
}
