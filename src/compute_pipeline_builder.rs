//! Builder for creating compute pipelines with a fluent API.

use wgpu::{
    BindGroupLayout, ComputePipeline, ComputePipelineDescriptor, Device,
    PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderModule, ShaderModuleDescriptor,
    ShaderSource,
};

/// Fluent builder for creating WebGPU compute pipelines.
///
/// Provides a chainable API for configuring compute pipeline parameters before creation.
///
/// # Required Methods Before `build()`
/// - `.shader()` - required
/// - `.entry_point()` - required
///
/// # Example
/// ```ignore
/// let pipeline = ComputePipelineBuilder::new(device)
///     .shader(SHADER_CODE, "compute_shader")
///     .entry_point("main")
///     .build("my_pipeline");
/// ```
pub struct ComputePipelineBuilder<'a> {
    device: &'a Device,
    bind_group_layouts: Vec<Option<&'a BindGroupLayout>>,
    immediate_size: u32,
    module: Option<ShaderModule>,
    entry_point: Option<&'a str>,
    compilation_options: PipelineCompilationOptions<'a>,
}

impl<'a> ComputePipelineBuilder<'a> {
    /// Creates a new compute pipeline builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - `.shader()` - required
    /// - `.entry_point()` - required
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layouts: Vec::new(),
            immediate_size: 0,
            module: None,
            entry_point: None,
            compilation_options: PipelineCompilationOptions::default(),
        }
    }

    /// Adds a bind group layout.
    ///
    /// # Arguments
    /// * `layout` - The bind group layout to add
    pub fn add_bind_group_layout(mut self, layout: &'a BindGroupLayout) -> Self {
        self.bind_group_layouts.push(Some(layout));
        self
    }

    /// Sets the compute shader source code.
    ///
    /// # Arguments
    /// * `wgsl_source` - The WGSL shader source code
    /// * `label` - Optional label for the shader module
    pub fn shader(mut self, wgsl_source: &'static str, label: &'a str) -> Self {
        self.module = Some(self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(wgsl_source.into()),
        }));
        self
    }

    /// Sets the entry point function name for the compute shader.
    ///
    /// # Arguments
    /// * `entry` - The entry point function name
    pub fn entry_point(mut self, entry: &'a str) -> Self {
        self.entry_point = Some(entry);
        self
    }

    /// Sets the compilation options for the compute pipeline.
    ///
    /// # Arguments
    /// * `options` - The compilation options
    pub fn compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.compilation_options = options;
        self
    }

    /// Builds the compute pipeline.
    ///
    /// # Panics
    /// Panics if shader module or entry point are not set.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> ComputePipeline {
        let module = self.module.expect(
            "ComputePipelineBuilder: shader module not set. Call .shader(wgsl_code, label) before build()",
        );
        let entry_point = self.entry_point.expect(
            "ComputePipelineBuilder: entry point not set. Call .entry_point(entry) before build()",
        );

        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &self.bind_group_layouts,
                immediate_size: self.immediate_size,
            });

        self.device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                module: &module,
                entry_point: Some(entry_point),
                compilation_options: self.compilation_options,
                cache: None,
            })
    }
}
