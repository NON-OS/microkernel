// NONOS Operating System (AGPL-3.0-or-later)

//! A DMA region backed by a broker grant, mapped coherent and below 4 GiB:
//! rtw89's buffer descriptors carry a 32-bit low address, and this driver
//! keeps the high bits zero (`ring::bd`), so a region above 4 GiB would be
//! truncated and the card would reach other memory.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::mem::{inside, DmaMem};

pub struct Grant {
    user_va: u64,
    device_addr: u64,
    length: usize,
}

/// Map `bytes`, whole pages, for the claim. `None` when the broker refused
/// or placed it where a 32-bit address cannot reach.
pub fn map_dma32(device_id: u64, epoch: u64, bytes: usize) -> Option<Grant> {
    let pages = (bytes as u64).div_ceil(0x1000) * 0x1000;
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let flags = MK_DMA_MAP_COHERENT | MK_DMA_MAP_DMA32;
    if mk_dma_map(device_id, epoch, pages, flags, &mut out) < 0 {
        return None;
    }
    let end = out.device_addr.checked_add(out.length);
    if end.is_none_or(|e| e > 1 << 32) {
        let _ = mk_dma_unmap(out.grant_id);
        return None;
    }
    // SAFETY: the broker mapped `length` coherent bytes at `user_va`, reached
    // by the card at `device_addr`, for as long as the claim lasts.
    Some(Grant { user_va: out.user_va, device_addr: out.device_addr, length: out.length as usize })
}

// SAFETY (both copies): the broker vouched for [user_va, user_va+length) as
// coherent memory of this process; `inside` put every byte within it, and
// volatile access keeps the copies ordered against the register kicks.
impl DmaMem for Grant {
    fn capacity(&self) -> usize {
        self.length
    }
    fn device_addr(&self) -> u64 {
        self.device_addr
    }
    fn write(&self, offset: usize, src: &[u8]) {
        if !inside(self.length, offset, src.len()) {
            return;
        }
        let base = (self.user_va as usize + offset) as *mut u8;
        for (i, b) in src.iter().enumerate() {
            unsafe { core::ptr::write_volatile(base.add(i), *b) }
        }
    }
    fn read(&self, offset: usize, dst: &mut [u8]) -> bool {
        if !inside(self.length, offset, dst.len()) {
            return false;
        }
        let base = (self.user_va as usize + offset) as *const u8;
        for (i, b) in dst.iter_mut().enumerate() {
            *b = unsafe { core::ptr::read_volatile(base.add(i)) };
        }
        true
    }
}
