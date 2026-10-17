// NONOS Operating System (AGPL-3.0-or-later)

//! A DMA region backed by a broker grant.

use super::mem::DmaMem;

/// A DMA region backed by a live broker grant: `user_va` is the mapped CPU
/// address, `device_addr` the physical address the card sees. The grant keeps
/// the mapping alive for the driver's lifetime.
pub struct Grant {
    user_va: u64,
    device_addr: u64,
    length: usize,
}

impl Grant {
    /// # Safety
    /// `user_va` must be the start of `length` bytes of coherent memory mapped
    /// for this process, `device_addr` the address the card reaches it at, for
    /// as long as this lives (as the broker's `mk_dma_map` gives them).
    pub const unsafe fn new(user_va: u64, device_addr: u64, length: usize) -> Self {
        Self { user_va, device_addr, length }
    }

    /// A CPU-readable view of the whole region. The receive path reads completed
    /// frames out of its buffers this way.
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: `new`'s caller vouched for [user_va, user_va+length) as
        // write-back coherent host memory for this grant's life, so a shared
        // byte view is sound; the card only ever writes frames read here.
        unsafe { core::slice::from_raw_parts(self.user_va as *const u8, self.length) }
    }
}

impl DmaMem for Grant {
    fn capacity(&self) -> usize {
        self.length
    }
    fn device_addr(&self) -> u64 {
        self.device_addr
    }
    /// A write that would run past the region writes nothing.
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        if offset.checked_add(src.len()).is_none_or(|end| end > self.length) {
            return;
        }
        let base = (self.user_va as usize + offset) as *mut u8;
        for (i, b) in src.iter().enumerate() {
            // SAFETY: `new`'s caller vouched for [user_va, user_va+length),
            // and the check above put offset+i inside it. Volatile, so the
            // copy is not reordered past the register kick that follows.
            unsafe { core::ptr::write_volatile(base.add(i), *b) }
        }
    }
}
