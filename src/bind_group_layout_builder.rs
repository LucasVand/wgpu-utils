//! Builder for creating bind group layouts with a fluent API.

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Device, SamplerBindingType, ShaderStages, StorageTextureAccess,
    TextureFormat, TextureSampleType, TextureViewDimension,
};

/// Fluent builder for creating WebGPU bind group layouts.
///
/// Provides a chainable API for configuring bind group layout entries.
///
/// # Required Methods Before `build()`
/// - At least one of: `.uniform()`, `.uniform_dyn()`, `.buffer()`, `.texture()`, `.sampler()`, `.sampler_comparison()`, `.storage_texture()`, or `.storage_texture_read()`
///
/// # Example
/// ```ignore
/// let layout = BindGroupLayoutBuilder::new(device)
///     .uniform(0, ShaderStages::VERTEX)
///     .texture(1, ShaderStages::FRAGMENT, TextureViewDimension::D2, TextureSampleType::Float { filterable: true })
///     .build("my_layout");
/// ```
pub struct BindGroupLayoutBuilder<'a> {
    entries: Vec<BindGroupLayoutEntry>,
    device: &'a Device,
}

impl<'a> BindGroupLayoutBuilder<'a> {
    /// Creates a new bind group layout builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - At least one of: `.uniform()`, `.uniform_dyn()`, `.buffer()`, `.texture()`, `.sampler()`, `.sampler_comparison()`, `.storage_texture()`, or `.storage_texture_read()`
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            entries: Vec::new(),
        }
    }

    fn push(mut self, entry: BindGroupLayoutEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Adds a uniform buffer binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    pub fn uniform(self, binding: u32, visibility: ShaderStages) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
    }

    /// Adds a dynamic uniform buffer binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    pub fn uniform_dyn(self, binding: u32, visibility: ShaderStages) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: None,
            },
            count: None,
        })
    }

    /// Adds a storage buffer binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    /// * `read_only` - Whether the buffer is read-only
    pub fn buffer(self, binding: u32, visibility: ShaderStages, read_only: bool) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Storage { read_only },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
    }

    /// Adds a texture binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    /// * `dimension` - The texture view dimension
    /// * `sample_type` - The texture sample type
    pub fn texture(
        self,
        binding: u32,
        visibility: ShaderStages,
        dimension: TextureViewDimension,
        sample_type: TextureSampleType,
    ) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Texture {
                sample_type,
                view_dimension: dimension,
                multisampled: false,
            },
            count: None,
        })
    }

    /// Adds a filtering sampler binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    pub fn sampler(self, binding: u32, visibility: ShaderStages) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            count: None,
        })
    }

    /// Adds a comparison sampler binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    pub fn sampler_comparison(self, binding: u32, visibility: ShaderStages) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::Sampler(SamplerBindingType::Comparison),
            count: None,
        })
    }

    /// Adds a read-write storage texture binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    /// * `format` - The texture format
    pub fn storage_texture(
        self,
        binding: u32,
        visibility: ShaderStages,
        format: TextureFormat,
    ) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format,
                view_dimension: TextureViewDimension::D3,
            },
            count: None,
        })
    }

    /// Adds a read-only storage texture binding.
    ///
    /// # Arguments
    /// * `binding` - The binding index
    /// * `visibility` - Which shader stages can access this binding
    /// * `format` - The texture format
    pub fn storage_texture_read(
        self,
        binding: u32,
        visibility: ShaderStages,
        format: TextureFormat,
    ) -> Self {
        self.push(BindGroupLayoutEntry {
            binding,
            visibility,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadOnly,
                format,
                view_dimension: TextureViewDimension::D3,
            },
            count: None,
        })
    }

    /// Builds the bind group layout.
    ///
    /// # Panics
    /// Panics if no entries have been added.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> BindGroupLayout {
        if self.entries.is_empty() {
            panic!(
                "BindGroupLayoutBuilder: no entries added. Call .uniform(), .uniform_dyn(), .buffer(), .texture(), .sampler(), .sampler_comparison(), or .storage_texture() to add bind group layout entries before build()"
            );
        }

        self.device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &self.entries,
            })
    }
}
