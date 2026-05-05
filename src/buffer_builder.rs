//! Builder for creating buffers with a fluent API.

use wgpu::{self, Buffer, BufferDescriptor, BufferUsages, Device, util::DeviceExt};

use crate::bytes_util;

/// Fluent builder for creating WebGPU buffers.
///
/// Provides a chainable API for configuring buffer parameters before creation.
///
/// # Required Methods Before `build()`
/// - `.usages()` - required
/// - Either `.contents()` or `.size()` - required
///
/// # Example
/// ```ignore
/// let buffer = BufferBuilder::new(device)
///     .usages(BufferUsages::UNIFORM | BufferUsages::COPY_DST)
///     .contents(&my_data)
///     .build("my_buffer");
/// ```
pub struct BufferBuilder<'a> {
    device: &'a Device,

    content: Option<&'a [u8]>,
    size: Option<u64>,
    usages: Option<BufferUsages>,
}

impl<'a> BufferBuilder<'a> {
    /// Creates a new buffer builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - `.usages()` - required
    /// - Either `.contents()` or `.size()` - required
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            content: None,
            size: None,
            usages: None,
        }
    }

    /// Sets the buffer content from any type.
    ///
    /// # Arguments
    /// * `contents` - Reference to the data to initialize the buffer with
    pub fn contents<T>(mut self, contents: &'a T) -> Self {
        self.content = Some(bytes_util::bytes_of(contents));
        self
    }

    /// Sets the buffer content from a byte slice.
    ///
    /// # Arguments
    /// * `contents` - The byte slice to initialize the buffer with
    pub fn contents_slice(mut self, contents: &'a [u8]) -> Self {
        self.content = Some(contents);
        self
    }

    /// Sets the buffer size in bytes.
    ///
    /// # Arguments
    /// * `size` - The size in bytes
    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the buffer usages.
    ///
    /// # Arguments
    /// * `usages` - The buffer usages flags
    pub fn usages(mut self, usages: BufferUsages) -> Self {
        self.usages = Some(usages);
        self
    }

    /// Builds the buffer.
    ///
    /// # Panics
    /// Panics if usages are not set, or neither content nor size are set.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> Buffer {
        let usages = self.usages.expect(
            "BufferBuilder: buffer usages not set. Call .usages(BufferUsages) before build()",
        );

        match self.content {
            Some(content) => self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: content,
                    usage: usages,
                }),
            None => {
                let size = self.size.expect(
                    "BufferBuilder: neither content nor size set. Call either .contents(data) or .size(bytes) before build()",
                );
                self.device.create_buffer(&BufferDescriptor {
                    label: Some(label),
                    size,
                    usage: usages,
                    mapped_at_creation: false,
                })
            }
        }
    }
}
