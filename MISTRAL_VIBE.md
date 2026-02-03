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
