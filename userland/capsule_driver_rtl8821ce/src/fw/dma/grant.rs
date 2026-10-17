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
    pub const fn new(user_va: u64, device_addr: u64, length: usize) -> Self {
        Self { user_va, device_addr, length }
    }

    /// A CPU-readable view of the whole region. The receive path reads completed
    /// frames out of its buffers this way.
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: the grant maps [user_va, user_va+length) as write-back coherent
        // host memory for the driver's lifetime, so a shared byte view is sound;
        // the card only ever writes frames the driver then reads here.
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
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        // SAFETY: the region is [user_va, user_va+length); callers keep
        // offset+src.len() within it, and the mapping is write-back coherent so
        // a byte copy is what the card later reads. Written volatile so the copy
        // is not reordered past the register kick that follows.
        let base = (self.user_va as usize + offset) as *mut u8;
        for (i, b) in src.iter().enumerate() {
            unsafe { core::ptr::write_volatile(base.add(i), *b) }
        }
    }
}
