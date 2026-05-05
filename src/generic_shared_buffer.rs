//! Generic shared buffer for efficiently managing multiple allocations in a single buffer.

use wgpu::{Buffer, BufferDescriptor, BufferSlice, BufferUsages, Device, Queue};

/// Represents a single allocation within a shared buffer.
#[derive(Clone)]
pub struct BufferAllocation {
    /// The byte offset of this allocation in the shared buffer
    pub offset: u64,
    /// The size of this allocation in bytes
    pub size: u64,
    /// Debug label for this allocation
    pub label: String,
}

/// Efficiently manages multiple data allocations within a single WebGPU buffer.
///
/// This buffer automatically handles alignment requirements and tracks allocations,
/// allowing you to pack multiple different data types into one buffer efficiently.
///
/// # Example
/// ```ignore
/// let mut shared_buffer = SharedBuffer::new(device, 4096);
/// let idx1 = shared_buffer.allocate(queue, &data1, "data1");
/// let idx2 = shared_buffer.allocate(queue, &data2, "data2");
/// 
/// // Update data later
/// shared_buffer.update(queue, idx1, &new_data1);
/// ```
pub struct SharedBuffer {
    buffer: Buffer,
    allocations: Vec<BufferAllocation>,
    current_offset: u64,
    max_size: u64,
}

impl SharedBuffer {
    const ALIGNMENT: u64 = 16; // 16-byte alignment for GPU

    /// Creates a new shared buffer with default usages (STORAGE | COPY_DST | UNIFORM).
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    /// * `size` - The total size of the buffer in bytes
    pub fn new(device: &Device, size: u64) -> Self {
        Self::with_usages(
            device,
            size,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        )
    }

    /// Creates a new shared buffer with custom usages.
    ///
    /// # Arguments
    /// * `device` - The WebGPU device
    /// * `size` - The total size of the buffer in bytes
    /// * `usages` - The buffer usages flags
    pub fn with_usages(device: &Device, size: u64, usages: BufferUsages) -> Self {
        let buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Generic Shared Buffer"),
            size,
            usage: usages,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            allocations: Vec::new(),
            current_offset: 0,
            max_size: size,
        }
    }

    /// Allocates space in the buffer and writes data to it.
    ///
    /// # Arguments
    /// * `queue` - The command queue
    /// * `data` - The data to write
    /// * `label` - Label for this allocation
    ///
    /// # Returns
    /// The allocation index for later reference
    pub fn allocate(&mut self, queue: &Queue, data: &[u8], label: &str) -> u64 {
        let aligned_offset = self.align_offset(self.current_offset);
        let data_size = data.len() as u64;

        if aligned_offset + data_size > self.max_size {
            panic!(
                "SharedBuffer: not enough space for '{}'. Need {} bytes at offset {}, but max is {}",
                label, data_size, aligned_offset, self.max_size
            );
        }

        queue.write_buffer(&self.buffer, aligned_offset, data);

        let allocation_index = self.allocations.len() as u64;
        self.allocations.push(BufferAllocation {
            offset: aligned_offset,
            size: data_size,
            label: label.to_string(),
        });

        self.current_offset = aligned_offset + data_size;
        self.current_offset = self.align_offset(self.current_offset);

        allocation_index
    }

    /// Allocates space in the buffer without writing initial content.
    ///
    /// # Arguments
    /// * `size` - The size to allocate in bytes
    /// * `label` - Label for this allocation
    ///
    /// # Returns
    /// The allocation index for later reference
    pub fn allocate_empty(&mut self, size: u64, label: &str) -> u64 {
        let aligned_offset = self.align_offset(self.current_offset);

        if aligned_offset + size > self.max_size {
            panic!(
                "SharedBuffer: not enough space for '{}'. Need {} bytes at offset {}, but max is {}",
                label, size, aligned_offset, self.max_size
            );
        }

        let allocation_index = self.allocations.len() as u64;
        self.allocations.push(BufferAllocation {
            offset: aligned_offset,
            size,
            label: label.to_string(),
        });

        self.current_offset = aligned_offset + size;
        self.current_offset = self.align_offset(self.current_offset);

        allocation_index
    }

    /// Allocates space with 256-byte alignment for dynamic uniform buffers and writes data.
    ///
    /// # Arguments
    /// * `queue` - The command queue
    /// * `data` - The data to write
    /// * `label` - Label for this allocation
    ///
    /// # Returns
    /// The allocation index for later reference
    pub fn allocate_uniform(&mut self, queue: &Queue, data: &[u8], label: &str) -> u64 {
        let aligned_offset = self.align_offset_uniform(self.current_offset);
        let data_size = data.len() as u64;

        if aligned_offset + data_size > self.max_size {
            panic!(
                "SharedBuffer: not enough space for uniform '{}'. Need {} bytes at offset {}, but max is {}",
                label, data_size, aligned_offset, self.max_size
            );
        }

        queue.write_buffer(&self.buffer, aligned_offset, data);

        let allocation_index = self.allocations.len() as u64;
        self.allocations.push(BufferAllocation {
            offset: aligned_offset,
            size: data_size,
            label: label.to_string(),
        });

        self.current_offset = aligned_offset + data_size;
        self.current_offset = self.align_offset_uniform(self.current_offset);

        allocation_index
    }

    /// Allocates space with 256-byte alignment without writing initial content.
    ///
    /// # Arguments
    /// * `size` - The size to allocate in bytes
    /// * `label` - Label for this allocation
    ///
    /// # Returns
    /// The allocation index for later reference
    pub fn allocate_uniform_empty(&mut self, size: u64, label: &str) -> u64 {
        let aligned_offset = self.align_offset_uniform(self.current_offset);

        if aligned_offset + size > self.max_size {
            panic!(
                "SharedBuffer: not enough space for uniform '{}'. Need {} bytes at offset {}, but max is {}",
                label, size, aligned_offset, self.max_size
            );
        }

        let allocation_index = self.allocations.len() as u64;
        self.allocations.push(BufferAllocation {
            offset: aligned_offset,
            size,
            label: label.to_string(),
        });

        self.current_offset = aligned_offset + size;
        self.current_offset = self.align_offset_uniform(self.current_offset);

        allocation_index
    }

    /// Updates an existing allocation with new data.
    ///
    /// # Panics
    /// Panics if the new data is larger than the allocated space.
    ///
    /// # Arguments
    /// * `queue` - The command queue
    /// * `index` - The allocation index
    /// * `new_data` - The new data to write
    pub fn update(&self, queue: &Queue, index: u64, new_data: &[u8]) {
        let alloc = self
            .allocations
            .get(index as usize)
            .unwrap_or_else(|| panic!("SharedBuffer: allocation index {} out of bounds", index));

        if new_data.len() as u64 > alloc.size {
            panic!(
                "SharedBuffer: new data ({} bytes) for '{}' exceeds allocated size ({} bytes)",
                new_data.len(),
                alloc.label,
                alloc.size
            );
        }

        queue.write_buffer(&self.buffer, alloc.offset, new_data);
    }

    /// Gets the byte offset of an allocation for use in shaders.
    ///
    /// # Arguments
    /// * `index` - The allocation index
    pub fn get_offset(&self, index: u64) -> u64 {
        self.allocations
            .get(index as usize)
            .unwrap_or_else(|| panic!("SharedBuffer: allocation index {} out of bounds", index))
            .offset
    }

    /// Gets the size of an allocation.
    ///
    /// # Arguments
    /// * `index` - The allocation index
    pub fn get_size(&self, index: u64) -> u64 {
        self.allocations
            .get(index as usize)
            .unwrap_or_else(|| panic!("SharedBuffer: allocation index {} out of bounds", index))
            .size
    }

    /// Gets a reference to the underlying GPU buffer.
    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Gets a buffer slice for an allocation.
    ///
    /// # Arguments
    /// * `index` - The allocation index
    pub fn get_slice(&self, index: u64) -> BufferSlice<'_> {
        let alloc = self
            .allocations
            .get(index as usize)
            .unwrap_or_else(|| panic!("SharedBuffer: allocation index {} out of bounds", index));

        self.buffer.slice(alloc.offset..alloc.offset + alloc.size)
    }

    /// Gets all allocations (for debugging or iteration).
    pub fn allocations(&self) -> &[BufferAllocation] {
        &self.allocations
    }

    /// Checks how much free space is available.
    pub fn available_space(&self) -> u64 {
        let next_aligned = self.align_offset(self.current_offset);
        if next_aligned >= self.max_size {
            0
        } else {
            self.max_size - next_aligned
        }
    }

    fn align_offset(&self, offset: u64) -> u64 {
        ((offset + Self::ALIGNMENT - 1) / Self::ALIGNMENT) * Self::ALIGNMENT
    }

    fn align_offset_uniform(&self, offset: u64) -> u64 {
        const UNIFORM_ALIGNMENT: u64 = 256;
        ((offset + UNIFORM_ALIGNMENT - 1) / UNIFORM_ALIGNMENT) * UNIFORM_ALIGNMENT
    }
}
