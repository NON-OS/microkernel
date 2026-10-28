// NONOS Operating System (AGPL-3.0-or-later)

//! The driver's DMA memory as the modelled WFDMA reaches it: by device
//! address, through the shim's grant registry. A read or write outside
//! every grant is a DMA the driver never mapped, and is not done.

use nonos_libc::dma_at;

pub fn read_mem(dev: u32, len: usize) -> Option<Vec<u8>> {
    let p = dma_at(dev, len)?;
    // SAFETY: `dma_at` found `len` bytes of a live grant at `p`.
    Some(unsafe { std::slice::from_raw_parts(p, len) }.to_vec())
}

pub fn write_mem(dev: u32, bytes: &[u8]) -> bool {
    let Some(p) = dma_at(dev, bytes.len()) else { return false };
    // SAFETY: as `read_mem`.
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len()) };
    true
}

pub fn word(dev: u32) -> u32 {
    read_mem(dev, 4).map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
