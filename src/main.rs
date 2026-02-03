use std::path::PathBuf;
use std::sync::OnceLock;
mod ui;
#[cfg(test)]
mod shadertoy_tests;

use bevy::{
    input_focus::{InputDispatchPlugin, tab_navigation::TabNavigationPlugin},
    prelude::*,
    render::render_resource::AsBindGroup,
    window::WindowResized,
};

use bevy_sprite_render::{Material2d, Material2dPlugin};
use clap::Parser;

use crate::ui::{SliderState, UiPlugin, read_slider_state, read_slider_state_for_dump};
use std::io::Write;

#[derive(Parser)]
struct Opt {
    #[arg(long, help = "Path to the shader file")]
    shader: String,
    
    #[arg(long, help = "Dump shader with hardcoded slider values for Shadertoy. Use '-' for stdout. Will watch for slider changes when running interactively")]
    dump_shadertoy: Option<String>,
}
static FRAGMENT: OnceLock<String> = OnceLock::new();

#[derive(Resource)]
pub struct ShaderPath(PathBuf);



fn main() -> AppExit {
    let opt = Opt::parse();
    let shader_path = PathBuf::from(&opt.shader);
    
    let config = read_slider_state(&shader_path).unwrap_or_default();
    
    // Strip "assets/" prefix for Bevy asset loader
    let fragment_path = if opt.shader.starts_with("assets/") {
        opt.shader.strip_prefix("assets/").unwrap().to_string()
    } else {
        opt.shader
    };
    FRAGMENT.set(fragment_path).unwrap();
    
    let mut app = App::new();
    app.insert_resource(config)
        .insert_resource(ShaderPath(shader_path.clone()))
        .add_plugins((
            DefaultPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin,
            Material2dPlugin::<CustomMaterial>::default(),
            UiPlugin,
            ShaderViewerPlugin,
        ));
    
    // Set up dump watcher if dump path is provided
    if let Some(output_path) = opt.dump_shadertoy {
        let dump_config = setup_dump_watcher(shader_path.clone(), output_path);
        app.insert_resource(dump_config)
            .add_systems(Update, check_for_changes_and_dump);
    }
    
    app.run()
}

#[derive(Default)]
struct ShaderViewerPlugin;

impl Plugin for ShaderViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
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

#[derive(Resource)]
struct DumpConfig {
    dump_path: String,
    shader_path: PathBuf,
    last_dump_time: std::time::SystemTime,
}

fn setup_dump_watcher(
    shader_path: PathBuf,
    dump_path: String,
) -> DumpConfig {
    // Initial dump
    let config = read_slider_state_for_dump(&shader_path).unwrap_or_default();
    if let Err(e) = dump_shadertoy_shader(&shader_path, &config, &dump_path) {
        eprintln!("Error dumping shader: {}", e);
    } else {
        println!("Initial shader dumped to {}", dump_path);
    }
    
    DumpConfig {
        dump_path,
        shader_path,
        last_dump_time: std::time::SystemTime::now(),
    }
}

fn check_for_changes_and_dump(
    slider_state: Res<SliderState>,
    dump_config: Res<DumpConfig>,
) {
    // Check if slider state has changed
    let slider_changed = slider_state.is_changed();
    
    // Debounce: only dump if enough time has passed since last dump
    let now = std::time::SystemTime::now();
    let debounce_duration = std::time::Duration::from_millis(500);
    
    if slider_changed {
        if let Ok(elapsed) = now.duration_since(dump_config.last_dump_time) {
            if elapsed >= debounce_duration {
                let config = read_slider_state_for_dump(&dump_config.shader_path).unwrap_or_default();
                if let Err(e) = dump_shadertoy_shader(&dump_config.shader_path, &config, &dump_config.dump_path) {
                    eprintln!("Error dumping shader after change: {}", e);
                } else {
                    println!("Shader updated and dumped to {}", dump_config.dump_path);
                }
            }
        }
    }
}

fn dump_shadertoy_shader(shader_path: &PathBuf, slider_state: &SliderState, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the original shader
    let shader_content = std::fs::read_to_string(shader_path)?;
    
    // Convert to Shadertoy format
    let shadertoy_content = convert_to_shadertoy(&shader_content, slider_state, shader_path);
    
    // Write to output
    if output_path == "-" {
        // Output to stdout
        println!("{}", shadertoy_content);
    } else {
        // Output to file
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(shadertoy_content.as_bytes())?;
    }
    
    Ok(())
}

pub fn convert_to_shadertoy(shader_content: &str, slider_state: &SliderState, original_path: &PathBuf) -> String {
    // Split the shader into lines for better processing
    let mut lines: Vec<String> = shader_content.lines().map(|s| s.to_string()).collect();
    
    // Remove version and layout declarations
    lines.retain(|line| {
        !line.trim().starts_with("#version") &&
        !line.trim().starts_with("layout(location = 0) in vec2 uv;") &&
        !line.trim().starts_with("layout(location = 0) out vec4 fragColor;") &&
        !line.trim().starts_with("layout(set = 2, binding = 0) uniform float iTime;") &&
        !line.trim().starts_with("layout(set = 2, binding = 1) uniform vec2 iResolution;") &&
        !line.trim().starts_with("layout(set = 2, binding = 2) uniform vec4 slider1;") &&
        !line.trim().starts_with("layout(set = 2, binding = 3) uniform vec4 slider2;") &&
        !line.trim().starts_with("layout(set = 2, binding = 4) uniform vec4 slider3;") &&
        !line.trim().starts_with("layout(set = 2, binding = 5) uniform vec4 slider4;") &&
        !line.trim().starts_with("layout(set = 2, binding = 6) uniform vec4 slider5;")
    });
    
    // Find and process the main function
    let mut main_function_start = None;
    let mut main_function_end = None;
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "void main() {" {
            main_function_start = Some(i);
        }
        if line.trim() == "}" && main_function_start.is_some() && main_function_end.is_none() {
            main_function_end = Some(i);
        }
    }
    
    // Extract the main function body
    let mut main_body = Vec::new();
    if let (Some(start), Some(end)) = (main_function_start, main_function_end) {
        for i in start + 1..end {
            main_body.push(lines[i].clone());
        }
    }
    
    // Add Shadertoy header and footer
    let mut shadertoy_shader = String::new();
    shadertoy_shader.push_str(&format!(
        "// Generated by bevy_shader_runner shadertoy dump\n// Original shader: {}\n\n",
        original_path.display()
    ));
    
    // Add the hardcoded slider values at the top
    shadertoy_shader.push_str(&format_hardcoded_value("slider1", slider_state.sliders_1));
    shadertoy_shader.push_str("\n");
    shadertoy_shader.push_str(&format_hardcoded_value("slider2", slider_state.sliders_2));
    shadertoy_shader.push_str("\n");
    shadertoy_shader.push_str(&format_hardcoded_value("slider3", slider_state.sliders_3));
    shadertoy_shader.push_str("\n");
    shadertoy_shader.push_str(&format_hardcoded_value("slider4", slider_state.sliders_4));
    shadertoy_shader.push_str("\n");
    shadertoy_shader.push_str(&format_hardcoded_value("slider5", slider_state.sliders_5));
    shadertoy_shader.push_str("\n\n");
    
    // Add any helper functions (lines before main function)
    if let Some(start) = main_function_start {
        for line in &lines[..start] {
            if !line.trim().is_empty() {
                shadertoy_shader.push_str(line);
                shadertoy_shader.push_str("\n");
            }
        }
    }
    
    // Add the Shadertoy main function
    shadertoy_shader.push_str("void mainImage(out vec4 fragColor, in vec2 fragCoord) {\n");
    shadertoy_shader.push_str("    vec2 uv = fragCoord / iResolution.xy;\n");
    
    // Add the main function body
    for line in main_body {
        shadertoy_shader.push_str("    ");
        shadertoy_shader.push_str(&line);
        shadertoy_shader.push_str("\n");
    }
    
    shadertoy_shader.push_str("}\n");
    
    shadertoy_shader
}

fn format_hardcoded_value(name: &str, value: Vec4) -> String {
    format!("const vec4 {} = vec4({}, {}, {}, {});", name, value.x, value.y, value.z, value.w)
}
