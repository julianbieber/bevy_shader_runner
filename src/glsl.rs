//! Loads GLSL `.vert` and `.frag` files as [`Shader`] assets by translating them to WGSL.

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    shader::Shader,
};
use naga::{
    ShaderStage,
    back::wgsl::WriterFlags,
    front::glsl::{Frontend, Options},
    valid::{Capabilities, ValidationFlags, Validator},
};

/// Registers the GLSL shader loader, so a [`Material2d`](bevy::sprite_render::Material2d)
/// may name a `.vert` or `.frag` asset as its shader.
pub struct GlslShaderPlugin;

impl Plugin for GlslShaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(GlslShaderLoader);
    }
}

#[derive(TypePath)]
struct GlslShaderLoader;

#[derive(Debug, thiserror::Error)]
enum GlslShaderLoaderError {
    #[error("could not read shader: {0}")]
    Io(#[from] std::io::Error),
    #[error("shader is not valid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    Translate(String),
}

impl AssetLoader for GlslShaderLoader {
    type Asset = Shader;
    type Settings = ();
    type Error = GlslShaderLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Shader, Self::Error> {
        let stage = match load_context
            .path()
            .path()
            .extension()
            .and_then(|e| e.to_str())
        {
            Some("vert") => ShaderStage::Vertex,
            _ => ShaderStage::Fragment,
        };
        let path = load_context.path().to_string();
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let wgsl = glsl_to_wgsl(&String::from_utf8(bytes)?, stage)
            .map_err(|e| GlslShaderLoaderError::Translate(format!("{path}: {e}")))?;
        Ok(Shader::from_wgsl(wgsl, path))
    }

    fn extensions(&self) -> &[&str] {
        &["vert", "frag"]
    }
}

/// Translates one GLSL shader stage to WGSL source.
///
/// Errors carry the frontend's or validator's message, rendered against `source`.
pub fn glsl_to_wgsl(source: &str, stage: ShaderStage) -> Result<String, String> {
    let module = Frontend::default()
        .parse(&Options::from(stage), source)
        .map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all())
        .validate(&module)
        .map_err(|e| e.emit_to_string(source))?;
    naga::back::wgsl::write_string(&module, &info, WriterFlags::empty()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pins that every GLSL shader shipped in assets still translates, now that bevy no longer reads GLSL itself.
    #[test]
    fn every_asset_shader_translates() {
        let mut seen = 0;
        for entry in std::fs::read_dir("assets/shaders").unwrap() {
            let path = entry.unwrap().path();
            let stage = match path.extension().and_then(|e| e.to_str()) {
                Some("vert") => ShaderStage::Vertex,
                Some("frag") => ShaderStage::Fragment,
                _ => continue,
            };
            let source = std::fs::read_to_string(&path).unwrap();
            if let Err(e) = glsl_to_wgsl(&source, stage) {
                panic!("{}: {e}", path.display());
            }
            seen += 1;
        }
        assert!(seen > 0);
    }
}
