//! Builder for creating command encoders with a fluent API.

use wgpu::{CommandEncoder, CommandEncoderDescriptor, Device};

/// Fluent builder for creating WebGPU command encoders.
pub struct CommandEncoderBuilder<'a> {
    device: &'a Device,
    label: Option<&'a str>,
}

impl<'a> CommandEncoderBuilder<'a> {
    /// Creates a new command encoder builder.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            label: None,
        }
    }

    /// Sets the label for the command encoder.
    ///
    /// # Arguments
    /// * `label` - The label for debugging
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Builds the command encoder.
    pub fn build(self) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: self.label })
    }
}
