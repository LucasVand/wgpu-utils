/// Convert a reference to any type into a byte slice.
///
/// # Safety
/// This function is safe for any type T that is properly initialized.
/// The returned slice is valid for the lifetime of the reference.
pub fn bytes_of<T: ?Sized>(val: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts((val as *const T) as *const u8, std::mem::size_of_val(val))
    }
}

/// Convert a slice of any type into a byte slice.
///
/// # Safety
/// This function is safe for any slice of properly initialized types.
/// The returned slice is valid for the lifetime of the input slice.
#[allow(dead_code)]
pub fn cast_slice<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * std::mem::size_of::<T>(),
        )
    }
}
