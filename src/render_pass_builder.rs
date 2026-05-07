//! Builder for creating render passes with a fluent API.

use std::num::NonZeroU32;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, QuerySet, RenderPass, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPassTimestampWrites, StoreOp,
    TextureView,
};

/// Builder for individual color attachments within a render pass.
///
/// Allows fine-grained configuration of each color attachment's properties:
/// view, clear color, load/store operations, resolve target, and depth slice.
pub struct ColorAttachmentBuilder<'a> {
    render_pass_builder: RenderPassBuilder<'a>,
    view: Option<&'a TextureView>,
    load_op: Option<LoadOp<Color>>,
    store_op: StoreOp,
    resolve_target: Option<&'a TextureView>,
    depth_slice: Option<u32>,
}

/// Fluent builder for creating WebGPU render passes.
///
/// Provides a chainable API for configuring render pass parameters, including color attachments,
/// depth/stencil attachments, and other rendering options. Allows fine-grained control over
/// each color attachment using the nested `ColorAttachmentBuilder`.
///
/// # Required Methods Before `build()`
/// - `.add_color_attachment()` - required (at least one)
///
/// # Example
/// ```ignore
/// let render_pass = RenderPassBuilder::new()
///     .label("main_pass")
///     .add_color_attachment()
///     .view(&color_view)
///     .clear(Color::BLACK)
///     .finalize()
///     .build(encoder);
/// ```
pub struct RenderPassBuilder<'a> {
    label: Option<&'a str>,
    color_attachments: Vec<Option<RenderPassColorAttachment<'a>>>,
    depth_view: Option<&'a TextureView>,
    depth_load_op: Option<LoadOp<f32>>,
    depth_store: bool,
    stencil_load_op: Option<LoadOp<u32>>,
    stencil_store: bool,
    timestamp_writes: Option<RenderPassTimestampWrites<'a>>,
    occlusion_query_set: Option<&'a QuerySet>,
    multiview_mask: Option<NonZeroU32>,
}

impl<'a> RenderPassBuilder<'a> {
    /// Creates a new render pass builder.
    ///
    /// # Example
    /// ```ignore
    /// let builder = RenderPassBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            label: None,
            color_attachments: Vec::new(),
            depth_view: None,
            depth_load_op: None,
            depth_store: true,
            stencil_load_op: None,
            stencil_store: true,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        }
    }

    /// Sets the label for the render pass.
    ///
    /// # Arguments
    /// * `label` - The label for debugging
    ///
    /// # Example
    /// ```ignore
    /// builder.label("main_render_pass")
    /// ```
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Starts building a new color attachment with fine-grained control.
    ///
    /// Returns a `ColorAttachmentBuilder` that allows configuring individual fields
    /// before adding it to the render pass with `.finalize()`.
    ///
    /// # Example
    /// ```ignore
    /// builder
    ///     .add_color_attachment()
    ///     .view(&color_view)
    ///     .clear(Color::BLACK)
    ///     .finalize()
    ///     .add_color_attachment()
    ///     .view(&second_view)
    ///     .load()
    ///     .finalize()
    /// ```
    pub fn add_color_attachment(self) -> ColorAttachmentBuilder<'a> {
        ColorAttachmentBuilder {
            render_pass_builder: self,
            view: None,
            load_op: None,
            store_op: StoreOp::Store,
            resolve_target: None,
            depth_slice: None,
        }
    }

    /// Sets the depth texture view for the render pass.
    ///
    /// # Arguments
    /// * `view` - The depth texture view
    ///
    /// # Example
    /// ```ignore
    /// builder.depth_view(&depth_texture_view)
    /// ```
    pub fn depth_view(mut self, view: &'a TextureView) -> Self {
        self.depth_view = Some(view);
        self
    }

    /// Sets the depth load operation.
    ///
    /// # Arguments
    /// * `load_op` - The load operation for depth (e.g., `LoadOp::Clear(1.0)` or `LoadOp::Load`)
    ///
    /// # Example
    /// ```ignore
    /// builder.depth_load_op(LoadOp::Clear(1.0))
    /// ```
    pub fn depth_load_op(mut self, load_op: LoadOp<f32>) -> Self {
        self.depth_load_op = Some(load_op);
        self
    }

    /// Sets whether to store depth data after the render pass.
    ///
    /// # Arguments
    /// * `store` - true to store depth data, false to discard
    ///
    /// # Example
    /// ```ignore
    /// builder.depth_store(true)
    /// ```
    pub fn depth_store(mut self, store: bool) -> Self {
        self.depth_store = store;
        self
    }

    /// Sets the stencil load operation.
    ///
    /// # Arguments
    /// * `load_op` - The load operation for stencil (e.g., `LoadOp::Clear(0)` or `LoadOp::Load`)
    ///
    /// # Example
    /// ```ignore
    /// builder.stencil_load_op(LoadOp::Clear(0))
    /// ```
    pub fn stencil_load_op(mut self, load_op: LoadOp<u32>) -> Self {
        self.stencil_load_op = Some(load_op);
        self
    }

    /// Sets whether to store stencil data after the render pass.
    ///
    /// # Arguments
    /// * `store` - true to store stencil data, false to discard
    ///
    /// # Example
    /// ```ignore
    /// builder.stencil_store(true)
    /// ```
    pub fn stencil_store(mut self, store: bool) -> Self {
        self.stencil_store = store;
        self
    }

    /// Convenience method to configure both depth and stencil for clearing.
    ///
    /// Sets depth load to `LoadOp::Clear(1.0)` and stencil load to `LoadOp::Clear(0)`,
    /// with both storing enabled.
    ///
    /// # Arguments
    /// * `view` - The depth/stencil texture view
    ///
    /// # Example
    /// ```ignore
    /// builder.depth_stencil_clear(&depth_stencil_view)
    /// ```
    pub fn depth_stencil_clear(mut self, view: &'a TextureView) -> Self {
        self.depth_view = Some(view);
        self.depth_load_op = Some(LoadOp::Clear(1.0));
        self.depth_store = true;
        self.stencil_load_op = Some(LoadOp::Clear(0));
        self.stencil_store = true;
        self
    }

    /// Convenience method to configure depth/stencil for loading previous data.
    ///
    /// Sets depth load to `LoadOp::Load` and stencil load to `LoadOp::Load`,
    /// with both storing enabled.
    ///
    /// # Arguments
    /// * `view` - The depth/stencil texture view
    ///
    /// # Example
    /// ```ignore
    /// builder.depth_stencil_load(&depth_stencil_view)
    /// ```
    pub fn depth_stencil_load(mut self, view: &'a TextureView) -> Self {
        self.depth_view = Some(view);
        self.depth_load_op = Some(LoadOp::Load);
        self.depth_store = true;
        self.stencil_load_op = Some(LoadOp::Load);
        self.stencil_store = true;
        self
    }

    /// Sets timestamp writes for the render pass.
    ///
    /// # Arguments
    /// * `timestamp_writes` - The timestamp write configuration
    ///
    /// # Example
    /// ```ignore
    /// builder.timestamp_writes(RenderPassTimestampWrites {
    ///     query_set: &query_set,
    ///     beginning_of_pass_write_index: Some(0),
    ///     end_of_pass_write_index: Some(1),
    /// })
    /// ```
    pub fn timestamp_writes(mut self, timestamp_writes: RenderPassTimestampWrites<'a>) -> Self {
        self.timestamp_writes = Some(timestamp_writes);
        self
    }

    /// Sets the occlusion query set for the render pass.
    ///
    /// # Arguments
    /// * `query_set` - The query set for occlusion queries
    ///
    /// # Example
    /// ```ignore
    /// builder.occlusion_query_set(&query_set)
    /// ```
    pub fn occlusion_query_set(mut self, query_set: &'a QuerySet) -> Self {
        self.occlusion_query_set = Some(query_set);
        self
    }

    /// Sets the multiview mask for the render pass.
    ///
    /// The multiview mask determines which views in a multiview texture array to render to.
    /// A value of 0 means standard rendering without multiview.
    ///
    /// # Arguments
    /// * `mask` - The multiview mask as a NonZeroU32, or None for standard rendering
    ///
    /// # Example
    /// ```ignore
    /// builder.multiview_mask(30)  // Render to views 0 and 1
    /// ```
    pub fn multiview_mask(mut self, mask: u32) -> Self {
        self.multiview_mask = NonZeroU32::new(mask);
        self
    }

    /// Builds the render pass.
    ///
    /// # Arguments
    /// * `encoder` - The command encoder to record the pass into
    ///
    /// # Panics
    /// Panics if no color attachments have been added before calling `build()`.
    ///
    /// # Required Methods Before `build()`
    /// - `.color_attachment()` or `.color_attachments()` - at least one must be called
    ///
    /// # Example
    /// ```ignore
    /// let render_pass = builder
    ///     .color_attachment(color_attachment)
    ///     .build(encoder);
    /// ```
    pub fn build(self, encoder: &'a mut CommandEncoder) -> RenderPass<'a> {
        assert!(
            !self.color_attachments.is_empty(),
            "At least one color attachment is required"
        );

        let depth_stencil_attachment = if let Some(view) = self.depth_view {
            let mut depth_ops = None;
            let mut stencil_ops = None;

            if let Some(load_op) = self.depth_load_op {
                depth_ops = Some(Operations {
                    load: load_op,
                    store: if self.depth_store {
                        StoreOp::Store
                    } else {
                        StoreOp::Discard
                    },
                });
            }

            if let Some(load_op) = self.stencil_load_op {
                stencil_ops = Some(Operations {
                    load: load_op,
                    store: if self.stencil_store {
                        StoreOp::Store
                    } else {
                        StoreOp::Discard
                    },
                });
            }

            Some(RenderPassDepthStencilAttachment {
                view,
                depth_ops,
                stencil_ops,
            })
        } else {
            None
        };

        encoder.begin_render_pass(&RenderPassDescriptor {
            label: self.label,
            color_attachments: &self.color_attachments,
            depth_stencil_attachment,
            timestamp_writes: self.timestamp_writes,
            occlusion_query_set: self.occlusion_query_set,
            multiview_mask: self.multiview_mask,
        })
    }
}

impl<'a> Default for RenderPassBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColorAttachmentBuilder<'a> {
    /// Sets the texture view for this color attachment.
    ///
    /// # Arguments
    /// * `view` - The texture view to render into
    ///
    /// # Example
    /// ```ignore
    /// .view(&color_view)
    /// ```
    pub fn view(mut self, view: &'a TextureView) -> Self {
        self.view = Some(view);
        self
    }

    /// Sets a clear color for this attachment.
    ///
    /// This sets the load operation to `LoadOp::Clear` with the given color.
    ///
    /// # Arguments
    /// * `color` - The color to clear with
    ///
    /// # Example
    /// ```ignore
    /// .clear(Color::BLACK)
    /// ```
    pub fn clear(mut self, color: Color) -> Self {
        self.load_op = Some(LoadOp::Clear(color));
        self
    }

    /// Sets the load operation to load existing data.
    ///
    /// This sets the load operation to `LoadOp::Load`.
    ///
    /// # Example
    /// ```ignore
    /// .load()
    /// ```
    pub fn load(mut self) -> Self {
        self.load_op = Some(LoadOp::Load);
        self
    }

    /// Sets whether to store the attachment after rendering.
    ///
    /// # Arguments
    /// * `store` - true to store, false to discard
    ///
    /// # Example
    /// ```ignore
    /// .store(true)
    /// ```
    pub fn store(mut self, store: bool) -> Self {
        self.store_op = if store {
            StoreOp::Store
        } else {
            StoreOp::Discard
        };
        self
    }

    /// Sets the resolve target for MSAA.
    ///
    /// # Arguments
    /// * `target` - The resolve target texture view
    ///
    /// # Example
    /// ```ignore
    /// .resolve_target(&msaa_resolve_view)
    /// ```
    pub fn resolve_target(mut self, target: &'a TextureView) -> Self {
        self.resolve_target = Some(target);
        self
    }

    /// Sets the depth slice for this color attachment.
    ///
    /// Used when rendering to a specific layer of a 3D texture or 2D array texture.
    ///
    /// # Arguments
    /// * `slice` - The depth slice index
    ///
    /// # Example
    /// ```ignore
    /// .depth_slice(0)
    /// ```
    pub fn depth_slice(mut self, slice: u32) -> Self {
        self.depth_slice = Some(slice);
        self
    }

    /// Finalizes this color attachment and returns to the render pass builder.
    ///
    /// # Panics
    /// Panics if the texture view has not been set with `.view()`.
    ///
    /// # Example
    /// ```ignore
    /// .finalize()
    /// ```
    pub fn finalize(mut self) -> RenderPassBuilder<'a> {
        let view = self.view.expect("Color attachment view must be set");
        let load_op = self.load_op.unwrap_or(LoadOp::Clear(Color::BLACK));

        let attachment = RenderPassColorAttachment {
            view,
            resolve_target: self.resolve_target,
            ops: Operations {
                load: load_op,
                store: self.store_op,
            },
            depth_slice: self.depth_slice,
        };

        self.render_pass_builder
            .color_attachments
            .push(Some(attachment));
        self.render_pass_builder
    }
}
