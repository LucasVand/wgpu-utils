# Builders

List of the available builder along with all methods associated

### RenderPipelineBuilder

Creates WebGPU render pipelines with full control over shading and rasterization.

**Required methods before `build()`:**

- `.shader()` - WGSL shader source
- `.primitive()` - Triangle, Line, or Point topology
- `.vertex_entry()` - Vertex shader entry point
- `.fragment_entry()` - Fragment shader entry point
- `.color_format()` - Output color format

**Optional methods:**

- `.add_bind_group_layout()` - Bind group layouts
- `.vertex_buffers()` - Vertex buffer layouts
- `.depth()` - Depth testing and writing
- `.blend_state()` - Custom blending
- `.vertex_compilation_options()` - Vertex shader compilation options
- `.fragment_compilation_options()` - Fragment shader compilation options
- `.compilation_options()` - Set both shader compilation options at once

### ComputePipelineBuilder

Creates WebGPU compute pipelines for GPU compute work.

**Required methods before `build()`:**

- `.shader()` - WGSL compute shader source
- `.entry_point()` - Compute shader entry point

**Optional methods:**

- `.add_bind_group_layout()` - Bind group layouts
- `.compilation_options()` - Shader compilation options

### BindGroupBuilder

Builds bind groups for shader resource binding.

**Required methods before `build()`:**

- At least one of `.buffer()`, `.buffer_slice()`, `.buffer_chunked()`, `.texture()`, or `.sampler()`

**Methods:**

- `.buffer()` - Bind entire buffer
- `.buffer_slice()` - Bind buffer slice
- `.buffer_chunked()` - Bind buffer chunk with size/offset
- `.texture()` - Bind texture view
- `.sampler()` - Bind sampler

### BindGroupLayoutBuilder

Creates bind group layouts that define resource binding structure.

**Required methods before `build()`:**

- At least one of `.uniform()`, `.uniform_dyn()`, `.buffer()`, `.texture()`, `.sampler()`, `.sampler_comparison()`, `.storage_texture()`, or `.storage_texture_read()`

**Methods:**

- `.uniform()` - Uniform buffer binding
- `.uniform_dyn()` - Dynamic uniform buffer binding
- `.buffer()` - Storage buffer binding
- `.texture()` - Texture binding
- `.sampler()` - Filtering sampler binding
- `.sampler_comparison()` - Comparison sampler binding
- `.storage_texture()` - Read-write storage texture binding
- `.storage_texture_read()` - Read-only storage texture binding

### BufferBuilder

Creates GPU buffers for storing data.

**Required methods before `build()`:**

- `.usages()` - Buffer usage flags
- Either `.contents()` or `.size()` - Initial data or size

**Methods:**

- `.contents()` - Initialize with data from any type
- `.contents_slice()` - Initialize with byte slice
- `.size()` - Allocate empty buffer of size
- `.usages()` - Set buffer usage flags

### TextureBuilder

Creates GPU textures for rendering and storage.

**Required methods before `build()`:**

- `.usages()` - Texture usage flags
- `.format()` - Pixel format
- `.dimension()` - 1D, 2D, or 3D
- `.size()` - Width, height, depth/array layers

**Optional methods:**

- `.view_formats()` - Compatible view formats
- `.sample_count()` - MSAA sample count
- `.mip_level_count()` - Mipmap levels

### CommandEncoderBuilder

Creates command encoders for recording GPU commands.

**Methods:**

- `.label()` - Optional debug label

### ComputePassBuilder

Creates compute passes for compute work.

**Methods:**

- `.label()` - Optional debug label
