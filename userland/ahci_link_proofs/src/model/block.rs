// NONOS Operating System (AGPL-3.0-or-later)
//! Host memory standing in for a port's registers, or a dead driver's frame.

use std::alloc::{alloc_zeroed, dealloc, Layout};

pub struct Block {
    ptr: *mut u8,
    layout: Layout,
}

impl Block {
    pub fn new(len: u64) -> Self {
        let layout = Layout::from_size_align(len as usize, 4096).expect("a page-aligned layout");
        // SAFETY: the layout has a nonzero size.
        let ptr = unsafe { alloc_zeroed(layout) };
        assert!(!ptr.is_null(), "out of host memory");
        Self { ptr, layout }
    }

    pub fn addr(&self) -> u64 {
        self.ptr as u64
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        // SAFETY: allocated in `new` with this layout.
        unsafe { dealloc(self.ptr, self.layout) }
    }
}
