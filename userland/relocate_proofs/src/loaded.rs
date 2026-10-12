// NONOS Operating System (AGPL-3.0-or-later)
//! A PIE copied into host memory as the arm64 boot protocol places an Image:
//! a 2 MiB aligned base plus text_offset, the link address's offset into its
//! own 2 MiB block. Then relocated there by the kernel's own code.

use std::alloc::{alloc_zeroed, dealloc, Layout};

use crate::elf::Pie;
use crate::relocate::apply::apply;
use crate::relocate::rela::{Placement, Refused, LOAD_ALIGN};

pub struct Loaded {
    pub base: u64,
    block: u64,
    layout: Layout,
}

impl Loaded {
    pub fn new(pie: &Pie, blob: &[u8]) -> Self {
        let text_offset = pie.link_base % LOAD_ALIGN;
        let size = (text_offset + pie.len) as usize;
        let layout = Layout::from_size_align(size, LOAD_ALIGN as usize).unwrap();
        // SAFETY: a non-zero size, freed in Drop with the same layout.
        let block = unsafe { alloc_zeroed(layout) } as u64;
        assert_ne!(block, 0);
        let base = block + text_offset;
        for s in &pie.segments {
            let to = (base + (s.vaddr - pie.link_base)) as *mut u8;
            let from = &blob[s.file.clone()];
            // SAFETY: the segment lies inside the image the layout covers.
            unsafe { std::ptr::copy_nonoverlapping(from.as_ptr(), to, from.len()) };
        }
        Self { base, block, layout }
    }

    pub fn relocate(&self, pie: &Pie) -> Result<usize, Refused> {
        let at = Placement { link_base: pie.link_base, load_base: self.base, len: pie.len };
        // SAFETY: the image is this allocation, which nothing else reads.
        unsafe { apply(&pie.relas, at) }
    }

    pub fn bytes(&self) -> &[u8] {
        let len = self.layout.size() - (self.base - self.block) as usize;
        // SAFETY: the image is the allocation's tail from `base`, initialised.
        unsafe { std::slice::from_raw_parts(self.base as *const u8, len) }
    }

    pub fn word(&self, offset: u64) -> u64 {
        u64::from_le_bytes(self.bytes()[offset as usize..offset as usize + 8].try_into().unwrap())
    }
}

impl Drop for Loaded {
    fn drop(&mut self) {
        // SAFETY: allocated in `new` with this layout.
        unsafe { dealloc(self.block as *mut u8, self.layout) }
    }
}
