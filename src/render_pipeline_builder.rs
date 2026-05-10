//! Builder for creating render pipelines with a fluent API.

use std::num::NonZeroU32;
use wgpu::{
    BindGroupLayout, BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Device, Face, FragmentState, FrontFace, IndexFormat, MultisampleState,
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
    
    // Primitive state fields
    topology: Option<PrimitiveTopology>,
    strip_index_format: Option<IndexFormat>,
    front_face: FrontFace,
    cull_mode: Option<Face>,
    unclipped_depth: bool,
    polygon_mode: PolygonMode,
    conservative: bool,

    vertex_buffer_layouts: &'a [VertexBufferLayout<'a>],

    depth_stencil: Option<DepthStencilState>,
    targets: Vec<Option<ColorTargetState>>,
    immediate_size: u32,
    vertex_compilation_options: PipelineCompilationOptions<'a>,
    fragment_compilation_options: PipelineCompilationOptions<'a>,
    multiview_mask: Option<NonZeroU32>,
}

impl<'a> RenderPipelineBuilder<'a> {
    /// Creates a new render pipeline builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - `.shader()` - required
    /// - `.topology()` - required
    /// - `.vertex_entry()` - required
    /// - `.fragment_entry()` - required
    /// - `.color_format()` - required
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layouts: Vec::new(),
            module: None,
            vertex: None,
            fragment: None,
            topology: None,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
            vertex_buffer_layouts: &[],
            targets: Vec::new(),
            depth_stencil: None,
            immediate_size: 0,
            vertex_compilation_options: PipelineCompilationOptions::default(),
            fragment_compilation_options: PipelineCompilationOptions::default(),
            multiview_mask: None,
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
    pub fn topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = Some(topology);
        self
    }

    /// Sets the strip index format for strip topologies.
    ///
    /// # Arguments
    /// * `format` - The index format (Uint16 or Uint32)
    pub fn strip_index_format(mut self, format: IndexFormat) -> Self {
        self.strip_index_format = Some(format);
        self
    }

    /// Sets the front face winding order.
    ///
    /// # Arguments
    /// * `winding` - The winding order (Ccw or Cw)
    pub fn front_face(mut self, winding: FrontFace) -> Self {
        self.front_face = winding;
        self
    }

    /// Sets the face culling mode.
    ///
    /// # Arguments
    /// * `cull` - The culling mode (None, Some(Face::Front), or Some(Face::Back))
    pub fn cull_mode(mut self, cull: Option<Face>) -> Self {
        self.cull_mode = cull;
        self
    }

    /// Enables or disables unclipped depth clamping.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable unclipped depth
    pub fn unclipped_depth(mut self, enabled: bool) -> Self {
        self.unclipped_depth = enabled;
        self
    }

    /// Sets the polygon rendering mode.
    ///
    /// # Arguments
    /// * `mode` - The polygon mode (Fill, Line, or Point)
    pub fn polygon_mode(mut self, mode: PolygonMode) -> Self {
        self.polygon_mode = mode;
        self
    }

    /// Enables or disables conservative rasterization.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable conservative rasterization
    pub fn conservative(mut self, enabled: bool) -> Self {
        self.conservative = enabled;
        self
    }

    /// Convenience method: Sets topology for triangle list rendering (default configuration).
    pub fn triangles(mut self) -> Self {
        self.topology = Some(PrimitiveTopology::TriangleList);
        self
    }

    /// Convenience method: Sets topology for triangle strip rendering.
    pub fn triangle_strip(mut self, index_format: IndexFormat) -> Self {
        self.topology = Some(PrimitiveTopology::TriangleStrip);
        self.strip_index_format = Some(index_format);
        self
    }

    /// Convenience method: Sets topology for line list rendering.
    pub fn lines(mut self) -> Self {
        self.topology = Some(PrimitiveTopology::LineList);
        self
    }

    /// Convenience method: Sets topology for line strip rendering.
    pub fn line_strip(mut self, index_format: IndexFormat) -> Self {
        self.topology = Some(PrimitiveTopology::LineStrip);
        self.strip_index_format = Some(index_format);
        self
    }

    /// Convenience method: Sets topology for point list rendering.
    pub fn points(mut self) -> Self {
        self.topology = Some(PrimitiveTopology::PointList);
        self
    }

    /// Convenience method: Enables back face culling (default front face: Ccw).
    pub fn cull_back(mut self) -> Self {
        self.cull_mode = Some(Face::Back);
        self
    }

    /// Convenience method: Enables front face culling (default front face: Ccw).
    pub fn cull_front(mut self) -> Self {
        self.cull_mode = Some(Face::Front);
        self
    }

    /// Convenience method: Disables face culling.
    pub fn no_cull(mut self) -> Self {
        self.cull_mode = None;
        self
    }

    /// Sets the vertex buffer layouts.
    ///
    /// # Arguments
    /// * `vertex_buffers` - slice of vertex buffer layouts
    pub fn vertex_buffers(mut self, vertex_buffers: &'a [VertexBufferLayout<'a>]) -> Self {
        self.vertex_buffer_layouts = vertex_buffers;
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

    /// Adds a render target color format. If this function is used multiple times then
    /// it allows for Multiple Render Targets
    ///
    /// # Arguments
    /// * `format` - The color target format
    pub fn add_target_format(mut self, format: TextureFormat) -> Self {
        self.targets.push(Some(ColorTargetState {
            format,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        }));
        self
    }

    /// Adds a render target with specific write mask and blend state.
    ///
    /// Allows fine-grained control over individual render target configuration.
    /// Use this for Multiple Render Targets with different blend and write settings.
    ///
    /// # Arguments
    /// * `format` - The color target format
    /// * `write_mask` - Which color channels to write (e.g., `ColorWrites::ALL`, `ColorWrites::RED | ColorWrites::BLUE`)
    /// * `blend` - Optional blend state (None for no blending)
    ///
    /// # Example
    /// ```ignore
    /// pipeline.add_target(
    ///     TextureFormat::Bgra8UnormSrgb,
    ///     ColorWrites::RGB,
    ///     Some(BlendState::ALPHA_BLENDING),
    /// )
    /// ```
    pub fn add_target(
        mut self,
        format: TextureFormat,
        write_mask: ColorWrites,
        blend: Option<BlendState>,
    ) -> Self {
        self.targets.push(Some(ColorTargetState {
            format,
            blend,
            write_mask,
        }));
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

    /// Sets the multiview mask for multiview rendering.
    ///
    /// The multiview mask determines which views in a multiview texture array to render to.
    /// A value of 0 means standard rendering without multiview.
    ///
    /// # Arguments
    /// * `mask` - The multiview mask as a NonZeroU32, or None for standard rendering
    ///
    /// # Example
    /// ```ignore
    /// pipeline.multiview_mask(30)  // Render to views 0 and 1
    /// ```
    pub fn multiview_mask(mut self, mask: u32) -> Self {
        self.multiview_mask = NonZeroU32::new(mask);
        self
    }

    /// Builds the render pipeline.
    ///
    /// # Panics
    /// Panics if shader module, topology, vertex entry, or fragment entry are not set.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> RenderPipeline {
        let module = self.module.expect(
            "PipelineBuilder: shader module not set. Call .shader(wgsl_code, label) before build()",
        );
        let topology = self
            .topology
            .expect("PipelineBuilder: primitive topology not set. Call .topology(PrimitiveTopology) or use a convenience method like .triangles() before build()");

        let primitive = PrimitiveState {
            topology,
            strip_index_format: self.strip_index_format,
            front_face: self.front_face,
            cull_mode: self.cull_mode,
            unclipped_depth: self.unclipped_depth,
            polygon_mode: self.polygon_mode,
            conservative: self.conservative,
        };

        let layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &self.bind_group_layouts,
                immediate_size: self.immediate_size,
            });

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: VertexState {
                    module: &module,
                    entry_point: self.vertex,
                    compilation_options: self.vertex_compilation_options,
                    buffers: &self.vertex_buffer_layouts,
                },
                primitive,
                depth_stencil: self.depth_stencil,
                multisample: MultisampleState::default(),
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: self.fragment,
                    compilation_options: self.fragment_compilation_options,
                    targets: &self.targets,
                }),
                multiview_mask: self.multiview_mask,
                cache: None,
            })
    }
}
