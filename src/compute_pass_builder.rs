//! Builder for creating compute passes with a fluent API.

use wgpu::{CommandEncoder, ComputePass, ComputePassDescriptor};

/// Fluent builder for creating WebGPU compute passes.
pub struct ComputePassBuilder<'a> {
    label: Option<&'a str>,
}

impl<'a> ComputePassBuilder<'a> {
    /// Creates a new compute pass builder.
    pub fn new() -> Self {
        Self { label: None }
    }

    /// Sets the label for the compute pass.
    ///
    /// # Arguments
    /// * `label` - The label for debugging
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Builds the compute pass.
    ///
    /// # Arguments
    /// * `encoder` - The command encoder to record the pass into
    pub fn build<'b>(self, encoder: &'b mut CommandEncoder) -> ComputePass<'b> {
        encoder.begin_compute_pass(&ComputePassDescriptor {
            label: self.label,
            timestamp_writes: None,
        })
    }
}

impl<'a> Default for ComputePassBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}
