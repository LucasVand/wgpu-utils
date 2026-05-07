# wgpu-utils

A collection of builder utilities for the [wgpu](https://github.com/gfx-rs/wgpu) WebGPU graphics library. Simplifies the creation of GPU pipelines, buffers, bind groups, and textures with a chainable, intuitive API.

## Features

- **Fluent API** - Chainable builder pattern for all GPU resources
- **Simple** - Easy and simple to learn and pick up
- **Tightly Knit** - Mimics the wgpu terminology and structure
- **Less Verbose** - Much less verbose than raw wgpu but still gives full control
- **Zero-dependency** - Only depends on wgpu reducing build times and dependency sizes

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wgpu-utils = "29.0.0"
wgpu = "0.19"
```

**Compatiable with wgpu versions 29.0.\***

## Quick Start

### Creating a Render Pipeline

```rust
use wgpu::{Device, ShaderStages, TextureFormat, PrimitiveTopology};
use wgpu_utils::RenderPipelineBuilder;

let pipeline = RenderPipelineBuilder::new(&device)
    .shader(SHADER_CODE, "main_shader")
    .primitive(PrimitiveTopology::TriangleList)
    .vertex_entry("vs_main")
    .fragment_entry("fs_main")
    .color_format(TextureFormat::Bgra8UnormSrgb)
    .build("render_pipeline");
```

### Creating a Compute Pipeline

```rust
use wgpu_utils::ComputePipelineBuilder;

let pipeline = ComputePipelineBuilder::new(&device)
    .shader(COMPUTE_SHADER, "compute_shader")
    .entry_point("main")
    .add_bind_group_layout(&bind_group_layout)
    .build("compute_pipeline");
```

### Creating a Render Pass

```rust
use wgpu::{Color};
use wgpu_utils::RenderPassBuilder;

let mut render_pass = RenderPassBuilder::new()
    .label("main_render_pass")
    .add_color_attachment()
    .view(&color_view)
    .clear(Color::BLACK)
    .finalize()
    .add_color_attachment()
    .view(&second_color_view)
    .load()
    .finalize()
    .depth_stencil_clear(&depth_view)
    .build(&mut encoder);
```

### Creating Bind Groups

```rust
use wgpu_utils::BindGroupBuilder;

let bind_group = BindGroupBuilder::new(&device, &layout)
    .buffer(0, &uniform_buffer)
    .texture(1, &my_texture)
    .sampler(2, &my_sampler)
    .build("bind_group");
```

### Creating Buffers

```rust
use wgpu::{BufferUsages};
use wgpu_utils::BufferBuilder;

// With initial data
let buffer = BufferBuilder::new(&device)
    .usages(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
    .contents(&data)
    .build("uniform_buffer");

// Empty buffer
let buffer = BufferBuilder::new(&device)
    .usages(BufferUsages::STORAGE)
    .size(1024) // buffer of size 1024 bytes
    .build("storage_buffer");
```

### Creating Textures

```rust
use wgpu::{TextureUsages, TextureDimension, TextureFormat};
use wgpu_utils::TextureBuilder;

let texture = TextureBuilder::new(&device)
    .usages(TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST)
    .format(TextureFormat::Bgra8UnormSrgb)
    .dimension(TextureDimension::D2)
    .size(1920, 1080, 1)
    .build("render_target");
```

### Shared Buffer (Memory Efficient)

The `SharedBuffer` allows you to pack multiple allocations into a single GPU buffer with automatic alignment:

```rust
use wgpu_utils::SharedBuffer;

let mut buffer = SharedBuffer::new(&device, 4096);

// Allocate and write data
let idx1 = buffer.allocate(&queue, &vertex_data, "vertices");
let idx2 = buffer.allocate(&queue, &index_data, "indices");

// Get offsets and sizes
let offset1 = buffer.get_offset(idx1);
let size1 = buffer.get_size(idx1);

// Update data later
buffer.update(&queue, idx1, &new_vertex_data);

// Check available space
let available = buffer.available_space();
```

## Examples

### Complete Render Setup

```rust
use wgpu::{Device, Queue, ShaderStages, TextureFormat, PrimitiveTopology, BufferUsages};
use wgpu_utils::{
    RenderPipelineBuilder, BindGroupLayoutBuilder, BindGroupBuilder,
    BufferBuilder, TextureBuilder
};

fn setup_rendering(device: &Device, queue: &Queue) {
    // Create bind group layout
    let bind_group_layout = BindGroupLayoutBuilder::new(device)
        .uniform(0, ShaderStages::VERTEX)
        .build("main_layout");

    // Create buffers
    let uniform_buffer = BufferBuilder::new(device)
        .usages(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
        .size(256)
        .build("uniforms");

    let vertex_buffer = BufferBuilder::new(device)
        .usages(BufferUsages::VERTEX | BufferUsages::COPY_DST)
        .contents(&vertex_data)
        .build("vertices");

    // Create bind group
    let bind_group = BindGroupBuilder::new(device, &bind_group_layout)
        .buffer(0, &uniform_buffer)
        .build("main_bind_group");

    // Create render pipeline
    let pipeline = RenderPipelineBuilder::new(device)
        .shader(SHADER_SOURCE, "main")
        .primitive(PrimitiveTopology::TriangleList)
        .vertex_entry("vs_main")
        .fragment_entry("fs_main")
        .add_bind_group_layout(&bind_group_layout)
        .color_format(TextureFormat::Bgra8UnormSrgb)
        .build("render_pipeline");
}
```

### Compute Work

```rust
use wgpu_utils::ComputePipelineBuilder;

let compute_pipeline = ComputePipelineBuilder::new(device)
    .shader(COMPUTE_SHADER, "compute")
    .entry_point("main")
    .add_bind_group_layout(&compute_layout)
    .build("compute_pipeline");
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related

- [wgpu](https://github.com/gfx-rs/wgpu) - The underlying WebGPU implementation
- [wgpu documentation](https://docs.rs/wgpu/)
