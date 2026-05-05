//! Builder for creating textures with a fluent API.

use wgpu::{self, Extent3d, Texture, TextureDimension, TextureFormat, Device, TextureUsages};

/// Fluent builder for creating WebGPU textures.
///
/// Provides a chainable API for configuring texture parameters before creation.
///
/// # Required Methods Before `build()`
/// - `.usages()` - required
/// - `.format()` - required
/// - `.dimension()` - required
/// - `.size()` - required
///
/// # Example
/// ```ignore
/// let texture = TextureBuilder::new(device)
///     .usages(TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST)
///     .format(TextureFormat::Bgra8UnormSrgb)
///     .dimension(TextureDimension::D2)
///     .size(800, 600, 1)
///     .build("my_texture");
/// ```
pub struct TextureBuilder<'a> {
    device: &'a Device,
    usages: Option<TextureUsages>,
    format: Option<TextureFormat>,
    view_formats: &'a [TextureFormat],
    mip_level_count: u32,
    sample_count: u32,
    dimension: Option<TextureDimension>,
    extent: Option<Extent3d>,
}

impl<'a> TextureBuilder<'a> {
    /// Creates a new texture builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    ///
    /// # Required Methods Before `build()`
    /// - `.usages()` - required
    /// - `.format()` - required
    /// - `.dimension()` - required
    /// - `.size()` - required
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            usages: None,
            format: None,
            view_formats: &[],
            sample_count: 1,
            mip_level_count: 1,
            dimension: None,
            extent: None,
        }
    }

    /// Sets the texture usages.
    ///
    /// # Arguments
    /// * `usages` - The texture usages flags
    pub fn usages(mut self, usages: TextureUsages) -> Self {
        self.usages = Some(usages);
        self
    }

    /// Sets the texture format.
    ///
    /// # Arguments
    /// * `format` - The texture format
    pub fn format(mut self, format: TextureFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Sets the view formats for reinterpreting the texture.
    ///
    /// # Arguments
    /// * `view_formats` - Slice of compatible view formats
    pub fn view_formats(mut self, view_formats: &'a [TextureFormat]) -> Self {
        self.view_formats = view_formats;
        self
    }

    /// Sets the sample count for multisampling.
    ///
    /// # Arguments
    /// * `sample_count` - The sample count (1, 4, or other values depending on support)
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    /// Sets the mip level count.
    ///
    /// # Arguments
    /// * `mip_level_count` - The number of mip levels
    pub fn mip_level_count(mut self, mip_level_count: u32) -> Self {
        self.mip_level_count = mip_level_count;
        self
    }

    /// Sets the texture dimension.
    ///
    /// # Arguments
    /// * `dimension` - The texture dimension (1D, 2D, or 3D)
    pub fn dimension(mut self, dimension: TextureDimension) -> Self {
        self.dimension = Some(dimension);
        self
    }

    /// Sets the texture size.
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `depth_or_array_layers` - Depth for 3D textures or array layers for 2D arrays
    pub fn size(mut self, width: u32, height: u32, depth_or_array_layers: u32) -> Self {
        self.extent = Some(Extent3d {
            width,
            height,
            depth_or_array_layers,
        });
        self
    }

    /// Builds the texture.
    ///
    /// # Panics
    /// Panics if size, dimension, format, or usages are not set.
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    pub fn build(self, label: &'a str) -> Texture {
        let extent = self
            .extent
            .expect("TextureBuilder: texture size not set. Call .size(width, height, depth_or_array_layers) before build()");
        let dimension = self
            .dimension
            .expect("TextureBuilder: texture dimension not set. Call .dimension(TextureDimension) before build()");
        let format = self.format.expect(
            "TextureBuilder: texture format not set. Call .format(TextureFormat) before build()",
        );
        let usages = self.usages.expect(
            "TextureBuilder: texture usages not set. Call .usages(TextureUsages) before build()",
        );

        self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: extent,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension,
            format,
            usage: usages,
            view_formats: self.view_formats,
        })
    }
}
