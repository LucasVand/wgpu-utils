//! Builder for creating bind groups with a fluent API.

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Buffer,
    BufferBinding, BufferSize, BufferSlice, Device, Sampler, TextureView,
};

/// Fluent builder for creating WebGPU bind groups.
///
/// Provides a chainable API for configuring bind group entries before creating the group.
///
/// # Required Methods Before `build()`
/// - At least one of: `.buffer()`, `.buffer_slice()`, `.buffer_chunked()`, `.texture()`, or `.sampler()`
///
/// # Example
/// ```ignore
/// let bind_group = BindGroupBuilder::new(device, layout)
///     .buffer(0, &my_buffer)
///     .texture(1, &my_texture)
///     .build("my_bind_group");
/// ```
pub struct BindGroupBuilder<'a> {
    device: &'a Device,
    layout: &'a BindGroupLayout,
    entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    /// Creates a new bind group builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    /// * `layout` - The bind group layout that defines the structure
    ///
    /// # Required Methods Before `build()`
    /// - At least one of: `.buffer()`, `.buffer_slice()`, `.buffer_chunked()`, `.texture()`, or `.sampler()`
    pub fn new(device: &'a Device, layout: &'a BindGroupLayout) -> Self {
        Self {
            device,
            layout,
            entries: Vec::new(),
        }
    }

    /// Adds a bind group entry directly.
    pub fn push(mut self, entry: BindGroupEntry<'a>) -> Self {
        self.entries.push(entry);
        self
    }

    /// Adds a buffer binding (entire buffer).
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `buffer` - The buffer to bind
    pub fn buffer(self, binding: u32, buffer: &'a Buffer) -> Self {
        self.push(BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        })
    }

    /// Adds a buffer binding for a specific slice.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `buffer_slice` - The buffer slice to bind
    pub fn buffer_slice(self, binding: u32, buffer_slice: BufferSlice<'a>) -> Self {
        self.push(BindGroupEntry {
            binding,
            resource: BindingResource::Buffer(BufferBinding {
                buffer: buffer_slice.buffer(),
                offset: buffer_slice.offset(),
                size: Some(buffer_slice.size()),
            }),
        })
    }

    /// Adds a chunked buffer binding with explicit size and offset.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `size` - The size of the buffer chunk in bytes
    /// * `offset` - The offset into the buffer in bytes
    /// * `buffer` - The buffer to bind
    pub fn buffer_chunked(self, binding: u32, size: u64, offset: u64, buffer: &'a Buffer) -> Self {
        self.push(BindGroupEntry {
            binding,
            resource: BindingResource::Buffer(BufferBinding {
                buffer,
                offset,
                size: Some(BufferSize::new(size).unwrap()),
            }),
        })
    }

    /// Adds a texture binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `texture_view` - The texture view to bind
    pub fn texture(self, binding: u32, texture_view: &'a TextureView) -> Self {
        self.push(BindGroupEntry {
            binding,
            resource: BindingResource::TextureView(texture_view),
        })
    }

    /// Adds a sampler binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `sampler` - The sampler to bind
    pub fn sampler(self, binding: u32, sampler: &'a Sampler) -> Self {
        self.push(BindGroupEntry {
            binding,
            resource: BindingResource::Sampler(sampler),
        })
    }

    /// Builds the bind group.
    ///
    /// # Panics
    /// Panics if no entries have been added.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> BindGroup {
        if self.entries.is_empty() {
            panic!(
                "BindGroupBuilder: no entries added. Call .buffer() or .buffer_chunked() to add bind group entries before build()"
            );
        }

        self.device.create_bind_group(&BindGroupDescriptor {
            label: Some(label),
            layout: self.layout,
            entries: &self.entries,
        })
    }
}
