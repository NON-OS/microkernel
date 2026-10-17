// NONOS Operating System (AGPL-3.0-or-later)

//! The DMA-memory seam the reserved-page staging is written against. Staging a
//! firmware chunk means placing bytes in a host buffer the card reads by bus
//! address, so the code needs both a CPU view (to write) and the device address
//! (to hand the card). A real buffer backs this with a broker `mk_dma_map` grant
//! (`user_va` for the writes, `device_addr` for the card); a host proof backs it
//! with a plain byte vector, so the exact bytes and descriptor the card would

/// A block of DMA-coherent memory: a CPU-writable region with a stable device
/// (bus) address. Offsets are in bytes from the start of the region.
pub trait DmaMem {
    /// Total size of the region in bytes.
    fn capacity(&self) -> usize;
    /// The device (bus) address of the region's start.
    fn device_addr(&self) -> u64;
    /// Copy `src` into the region at `offset`. A copy that would run past the
    /// region writes nothing.
    fn write_bytes(&self, offset: usize, src: &[u8]);
}
