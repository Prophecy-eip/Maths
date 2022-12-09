use std::ptr;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RawArray<T> {
    pub size: u64,
    pub items: *const T,
}

impl<T> RawArray<T>
where
    T: Clone,
{
    pub fn dehydrate(vec: Vec<T>) -> RawArray<T> {
        RawArray {
            size: vec.len() as u64,
            items: vec.as_ptr(),
        }
    }

    pub fn hydrate(raw_array: &RawArray<T>) -> Vec<T> {
        let mut vec = Vec::with_capacity(raw_array.size as usize);
        unsafe {
            ptr::copy_nonoverlapping(raw_array.items, vec.as_mut_ptr(), raw_array.size as usize);
            vec.set_len(raw_array.size as usize);
        }
        vec
    }
}
