//! Utilities for building WebGPU pipelines, buffers, and bind groups with a fluent API.

mod bind_group_builder;
mod bind_group_layout_builder;
mod buffer_builder;
mod bytes_util;
mod command_encoder_builder;
mod compute_pass_builder;
mod compute_pipeline_builder;
mod generic_shared_buffer;
mod render_pipeline_builder;
mod texture_builder;

pub use bind_group_builder::BindGroupBuilder;
pub use bind_group_layout_builder::BindGroupLayoutBuilder;
pub use buffer_builder::BufferBuilder;
pub use command_encoder_builder::CommandEncoderBuilder;
pub use compute_pass_builder::ComputePassBuilder;
pub use compute_pipeline_builder::ComputePipelineBuilder;
pub use generic_shared_buffer::{BufferAllocation, SharedBuffer};
pub use render_pipeline_builder::RenderPipelineBuilder;
pub use texture_builder::TextureBuilder;
