# Mistral Vibe Integration

This document describes the integration of Mistral Vibe with the bevy_shader_runner project.

## Overview

Mistral Vibe is an AI-powered coding assistant that helps with code exploration, modification, and task management within this Rust-based Bevy shader runner project.

## Project Structure

The bevy_shader_runner project has the following key components:

- **Main application**: `src/main.rs` - Entry point for the Bevy application
- **UI components**: `src/ui.rs` - User interface implementation
- **Shader assets**: `assets/shaders/` - Collection of shader files (WGSL, GLSL)
- **Configuration**: `assets/configs/` - Shader configuration files

## Key Features

1. **Shader Rendering**: The application loads and renders various shaders using Bevy's rendering pipeline
2. **UI Controls**: Provides interactive controls for shader parameters
3. **Configuration**: Supports loading shader configurations from JSON files

## Mistral Vibe Capabilities

Mistral Vibe can assist with:

- **Code Exploration**: Navigating the Rust/Bevy codebase
- **Shader Development**: Modifying and creating new shader files
- **UI Enhancements**: Improving the user interface components
- **Build Optimization**: Analyzing and optimizing build configurations
- **Error Diagnosis**: Identifying and fixing issues in the shader pipeline

## Usage Patterns

Common tasks that Mistral Vibe can help with:

1. **Adding new shaders**: Creating new shader files and integrating them into the application
2. **UI improvements**: Enhancing the control interface for shader parameters
3. **Performance tuning**: Optimizing shader code and rendering pipeline
4. **Cross-platform support**: Ensuring compatibility across different platforms

## Integration Points

- **Shader Pipeline**: Understanding how shaders are loaded and applied
- **Bevy ECS**: Working with Bevy's Entity Component System architecture
- **Asset Management**: Handling shader assets and configurations
- **Input Handling**: Managing user input for shader parameter control

## Best Practices

When working with this project through Mistral Vibe:

1. **Shader Development**: 
   - Test shaders incrementally
   - Use the existing shader examples as templates
   - Validate WGSL/GLSL syntax before integration

2. **UI Development**:
   - Follow Bevy's UI patterns
   - Maintain consistency with existing control layouts
   - Test across different window sizes

3. **Performance**:
   - Profile shader execution
   - Optimize uniform updates
   - Minimize texture bindings when possible

## Learnings from Shadertoy Integration

### Bevy Change Detection

The implementation of automatic shader dumping when sliders change provided valuable insights into Bevy's change detection system:

- **Resource Tracking**: Bevy's `is_changed()` method on resources is powerful for detecting state changes
- **System Integration**: Adding systems that respond to changes is straightforward with Bevy's ECS architecture
- **Debouncing**: Implementing debounce logic prevents excessive triggers during rapid UI interactions

### Shader Conversion Challenges

Converting Bevy GLSL shaders to Shadertoy format revealed several important considerations:

- **Uniform Handling**: Bevy uses specific layout bindings that need to be removed for Shadertoy compatibility
- **Function Preservation**: Helper functions must be carefully extracted and preserved during conversion
- **Main Function Transformation**: The conversion from `main()` to `mainImage()` requires proper UV coordinate setup

### Testing Strategy

Developing comprehensive unit tests for the shader conversion provided insights into effective testing approaches:

- **Input/Output Testing**: Verify that specific input patterns produce expected output structures
- **Edge Case Coverage**: Test complex shader structures with nested functions and multiple uniforms
- **Regression Prevention**: Tests serve as documentation and prevent future breaking changes

### CLI Design Patterns

Enhancing the CLI with the `--dump-shadertoy` argument demonstrated effective patterns:

- **Help Documentation**: Clear, descriptive help text improves user experience
- **Flexible Output**: Supporting both file output and stdout provides versatility
- **Progressive Enhancement**: Starting with basic functionality and adding features incrementally

### File System Integration

Working with shader assets and configuration files highlighted important patterns:

- **Path Handling**: Proper path manipulation is crucial for cross-platform compatibility
- **Config Management**: Separating runtime config from persistent storage prevents conflicts
- **Asset Loading**: Bevy's asset system requires specific path formats for proper loading

### Performance Considerations

Implementing real-time features revealed performance optimization opportunities:

- **Change Detection**: Only process changes when they actually occur, not on every frame
- **Debounce Timing**: Balancing responsiveness with performance through appropriate debounce delays
- **Resource Management**: Efficient resource usage prevents memory leaks in long-running applications

## Future Enhancement Opportunities

Based on this implementation, several areas for future improvement were identified:

1. **Shader Hot Reloading**: Extend change detection to monitor file system changes for live shader editing
2. **Advanced Conversion**: Support more complex shader patterns and GLSL features
3. **Export Formats**: Add support for additional shader platforms beyond Shadertoy
4. **UI Feedback**: Provide visual indicators when automatic exports occur
5. **Error Recovery**: Enhance error handling for malformed shaders and invalid configurations

## Justfile Integration

The project includes a comprehensive Justfile that provides valuable build and development automation:

### Key Justfile Commands

1. **Quality Assurance**
   - `just fmt`: Format check with strict Rust formatting rules
   - `just clippy`: Run Clippy lints with comprehensive checks
   - `just docs`: Documentation check with private item documentation
   - `just bevy-lints`: Bevy-specific linting for best practices

2. **Testing & Validation**
   - `just test`: Run all tests with locked dependencies
   - `just check-web`: Web compilation check with WASM target
   - `just all`: Run complete CI pipeline (fmt, docs, clippy, lints, tests)

3. **Development Workflow**
   - `just run <name>`: Run native shader with Bevy CLI
   - `just run-web`: Run web version with WASM compilation
   - `just new-shader <name>`: Create new shader from template

4. **Configuration**
   - Uses strict Rust flags: `-Dwarnings -Zshare-generics=y -Zthreads=0`
   - Supports WASM compilation with `wasm32-unknown-unknown` target
   - Includes getrandom WASM configuration for web builds

### Justfile Best Practices

1. **Strict Mode**: Uses `set shell := ["bash", "-eu", "-o", "pipefail", "-c"]` for robust error handling
2. **Environment Isolation**: Sets `RUSTFLAGS` and `RUSTDOCFLAGS` consistently across all commands
3. **CI Integration**: The `all` target runs commands in CI order for consistent validation
4. **Template System**: Provides shader templates for rapid development
5. **Cross-Platform**: Supports both native and web compilation targets

### Integration with Current Implementation

The existing Shadertoy dump functionality could be enhanced by integrating with the Justfile:

1. **Add Shadertoy Test Target**: Create a `just test-shadertoy` command to validate conversion
2. **Automated Export**: Add `just export-shadertoy` for batch processing multiple shaders
3. **CI Integration**: Include Shadertoy validation in the `all` target for comprehensive testing
4. **Web Export**: Extend web compilation to include Shadertoy-compatible exports

### Example Justfile Enhancement

```justfile
# Test Shadertoy conversion
test-shadertoy:
    @cargo test --test shadertoy_tests

# Export all shaders to Shadertoy format
export-shadertoy:
    @for shader in assets/shaders/*.frag; do \
        cargo run -- --shader "$$shader" --dump-shadertoy "exports/$(basename "$$shader" .frag)-shadertoy.glsl"; \
    done

# Validate all Shadertoy exports
validate-shadertoy:
    @cargo test --test shadertoy_tests && \
     echo "Shadertoy conversion validated successfully"
```

This integration would provide a more robust development workflow and better CI/CD integration for the Shadertoy functionality.
