// NONOS Operating System (AGPL-3.0-or-later)

//! The DMA-memory seam the rings are written against: a CPU view to write
//! descriptors and frames, read what the card wrote, and the bus address
//! the card is handed. A broker grant backs it on the card; the proofs back
//! it with host memory the modelled chip reaches by the same bus address.

pub trait DmaMem {
    /// Size of the region in bytes.
    fn capacity(&self) -> usize;
    /// The bus address of the region's first byte.
    fn device_addr(&self) -> u64;
    /// Copy `src` in at `offset`. A copy that would run past the end writes
    /// nothing.
    fn write(&self, offset: usize, src: &[u8]);
    /// Copy out to `dst` from `offset`. A read past the end fills nothing and
    /// returns false.
    fn read(&self, offset: usize, dst: &mut [u8]) -> bool;

    /// The bus address of `offset`, when it lies inside the region.
    fn bus_at(&self, offset: usize) -> Option<u32> {
        let a = self.device_addr().checked_add(offset as u64)?;
        (offset < self.capacity() && a < (1 << 32)).then_some(a as u32)
    }
}

/// Whether `[offset, offset + len)` lies inside a region of `cap` bytes.
pub const fn inside(cap: usize, offset: usize, len: usize) -> bool {
    match offset.checked_add(len) {
        Some(end) => end <= cap,
        None => false,
    }
}
