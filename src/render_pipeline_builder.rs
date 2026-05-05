//! Builder for creating render pipelines with a fluent API.

use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Device, FragmentState, FrontFace, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    ShaderModuleDescriptor, ShaderSource, StencilState, TextureFormat, VertexBufferLayout,
    VertexState,
};

/// Fluent builder for creating WebGPU render pipelines.
///
/// Provides a chainable API for configuring render pipeline parameters, including shaders,
/// bind groups, vertex buffers, and rasterization state.
///
/// # Required Methods Before `build()`
/// - `.shader()` - required
/// - `.primitive()` - required
/// - `.vertex_entry()` - required
/// - `.fragment_entry()` - required
/// - `.color_format()` - required
///
/// # Example
/// ```ignore
/// let pipeline = RenderPipelineBuilder::new(device)
///     .shader(SHADER_CODE, "shaders")
///     .primitive(PrimitiveTopology::TriangleList)
///     .vertex_entry("vs_main")
///     .fragment_entry("fs_main")
///     .color_format(TextureFormat::Bgra8UnormSrgb)
///     .build("my_pipeline");
/// ```
pub struct RenderPipelineBuilder<'a> {
    device: &'a Device,
    bind_group_layouts: Vec<Option<&'a BindGroupLayout>>,

    module: Option<ShaderModule>,
    vertex: Option<&'a str>,
    fragment: Option<&'a str>,
    primitive: Option<PrimitiveState>,

    vertex_buffers: Vec<VertexBufferLayout<'a>>,

    depth_stencil: Option<DepthStencilState>,
    color_targets: Vec<Option<ColorTargetState>>,
    blend_state: Option<BlendState>,
    immediate_size: u32,
    vertex_compilation_options: PipelineCompilationOptions<'a>,
    fragment_compilation_options: PipelineCompilationOptions<'a>,
}

impl<'a> RenderPipelineBuilder<'a> {
    /// Creates a new render pipeline builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - `.shader()` - required
    /// - `.primitive()` - required
    /// - `.vertex_entry()` - required
    /// - `.fragment_entry()` - required
    /// - `.color_format()` - required
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layouts: Vec::new(),
            module: None,
            primitive: None,
            vertex: None,
            fragment: None,
            vertex_buffers: Vec::new(),
            color_targets: Vec::new(),
            depth_stencil: None,
            blend_state: None,
            immediate_size: 0,
            vertex_compilation_options: PipelineCompilationOptions::default(),
            fragment_compilation_options: PipelineCompilationOptions::default(),
        }
    }

    /// Sets the immediate push constant size.
    ///
    /// # Arguments
    /// * `size` - The immediate constant size
    pub fn immediate_size(mut self, size: u32) -> Self {
        self.immediate_size = size;
        self
    }

    /// Adds a bind group layout.
    ///
    /// # Arguments
    /// * `layout` - The bind group layout to add
    pub fn add_bind_group_layout(mut self, layout: &'a BindGroupLayout) -> Self {
        self.bind_group_layouts.push(Some(layout));
        self
    }

    /// Sets a pre-built shader module.
    ///
    /// # Arguments
    /// * `module` - The shader module
    pub fn shader_module(mut self, module: ShaderModule) -> Self {
        self.module = Some(module);
        self
    }

    /// Creates and sets a shader module from WGSL source code (alternative name).
    ///
    /// # Arguments
    /// * `naga` - The WGSL shader source code
    /// * `label` - Optional label for the shader module
    pub fn shader_naga(mut self, naga: &'a str, label: &'a str) -> Self {
        self.module = Some(self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(naga.into()),
        }));
        self
    }

    /// Creates and sets a shader module from WGSL source code.
    ///
    /// # Arguments
    /// * `module` - The WGSL shader source code
    /// * `label` - Optional label for the shader module
    pub fn shader(mut self, module: &'a str, label: &'a str) -> Self {
        self.module = Some(self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(module.into()),
        }));
        self
    }

    /// Sets the vertex shader entry point function name.
    ///
    /// # Arguments
    /// * `entry` - The vertex entry point function name
    pub fn vertex_entry(mut self, entry: &'a str) -> Self {
        self.vertex = Some(entry);
        self
    }

    /// Sets the fragment shader entry point function name.
    ///
    /// # Arguments
    /// * `entry` - The fragment entry point function name
    pub fn fragment_entry(mut self, entry: &'a str) -> Self {
        self.fragment = Some(entry);
        self
    }

    /// Sets the primitive topology.
    ///
    /// # Arguments
    /// * `topology` - The primitive topology (e.g., TriangleList, LineList, PointList)
    pub fn primitive(mut self, topology: PrimitiveTopology) -> Self {
        self.primitive = Some(PrimitiveState {
            topology,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        });
        self
    }

    /// Sets the vertex buffer layouts.
    ///
    /// # Arguments
    /// * `vertex_buffers` - Vector of vertex buffer layouts
    pub fn vertex_buffers(mut self, vertex_buffers: Vec<VertexBufferLayout<'a>>) -> Self {
        self.vertex_buffers = vertex_buffers;
        self
    }

    /// Sets the depth-stencil format.
    ///
    /// # Arguments
    /// * `format` - The depth-stencil texture format
    pub fn depth(mut self, format: TextureFormat) -> Self {
        self.depth_stencil = Some(DepthStencilState {
            format,
            depth_write_enabled: Some(true),
            depth_compare: Some(CompareFunction::Less),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        });
        self
    }

    /// Sets the render target color format.
    ///
    /// # Arguments
    /// * `format` - The color target format
    pub fn color_format(mut self, format: TextureFormat) -> Self {
        self.color_targets = vec![Some(ColorTargetState {
            format,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        })];
        self
    }

    /// Sets a custom blend state.
    ///
    /// # Arguments
    /// * `blend` - The blend state
    pub fn blend_state(mut self, blend: BlendState) -> Self {
        self.blend_state = Some(blend);
        self
    }

    /// Sets the vertex shader compilation options.
    ///
    /// # Arguments
    /// * `options` - The compilation options
    pub fn vertex_compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.vertex_compilation_options = options;
        self
    }

    /// Sets the fragment shader compilation options.
    ///
    /// # Arguments
    /// * `options` - The compilation options
    pub fn fragment_compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.fragment_compilation_options = options;
        self
    }

    /// Sets the compilation options for both vertex and fragment shaders.
    ///
    /// # Arguments
    /// * `options` - The compilation options
    pub fn compilation_options(mut self, options: PipelineCompilationOptions<'a>) -> Self {
        self.vertex_compilation_options = options.clone();
        self.fragment_compilation_options = options;
        self
    }

    /// Builds the render pipeline.
    ///
    /// # Panics
    /// Panics if shader module, primitive topology, vertex entry, or fragment entry are not set.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> RenderPipeline {
        let module = self.module.expect(
            "PipelineBuilder: shader module not set. Call .shader(wgsl_code, label) before build()",
        );
        let primitive = self
            .primitive
            .expect("PipelineBuilder: primitive topology not set. Call .primitive(PrimitiveTopology) before build()");

        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &self.bind_group_layouts,
                immediate_size: self.immediate_size,
            });

        // Apply blend state to color targets if it was set
        let color_targets = if let Some(blend) = self.blend_state {
            vec![Some(ColorTargetState {
                format: self
                    .color_targets
                    .first()
                    .and_then(|ct| ct.as_ref())
                    .map(|ct| ct.format)
                    .expect("RenderPipelineBuilder: color format not set"),
                blend: Some(blend),
                write_mask: ColorWrites::ALL,
            })]
        } else {
            self.color_targets
        };

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &module,
                    entry_point: self.vertex,
                    compilation_options: self.vertex_compilation_options,
                    buffers: &self.vertex_buffers,
                },
                primitive,
                depth_stencil: self.depth_stencil,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: self.fragment,
                    compilation_options: self.fragment_compilation_options,
                    targets: &color_targets,
                }),
                multiview_mask: None,
                cache: None,
            })
    }
}
