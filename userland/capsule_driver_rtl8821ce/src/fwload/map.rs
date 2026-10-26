// NONOS Operating System (AGPL-3.0-or-later)

//! The DMA regions the driver maps through the broker: the reserved-page
//! path's two here, the data path's four through `map_dma32` as well.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use super::FwLoadError;
use crate::fw::dma::{reachable, Grant};
use crate::fw::regs::TX_DESC_SIZE;
use crate::fw::MAX_CHUNK;
use crate::status;

/// Staging buffer: one DDMA chunk plus its 48-byte TX descriptor, rounded to
/// whole pages. Two pages cover the 0x1000-byte chunk and the descriptor.
const STAGE_BYTES: u64 = 0x2000;
/// The beacon-queue buffer-descriptor ring. One page is ample for the single
/// descriptor firmware download reuses.
const RING_BYTES: u64 = 0x1000;

// The staging buffer must hold the largest chunk plus its TX descriptor.
const _: () = assert!(STAGE_BYTES as usize >= MAX_CHUNK + TX_DESC_SIZE as usize);

/// The staging buffer and the beacon-queue ring.
pub(super) fn map_regions(device_id: u64, claim_epoch: u64) -> Result<(Grant, Grant), FwLoadError> {
    let Some(stage_buf) = map_dma32(device_id, claim_epoch, STAGE_BYTES) else {
        status::line(b"[rtl8821ce] fw: staging DMA map failed\n");
        return Err(FwLoadError::Dma);
    };
    let Some(ring) = map_dma32(device_id, claim_epoch, RING_BYTES) else {
        status::line(b"[rtl8821ce] fw: ring DMA map failed\n");
        return Err(FwLoadError::Dma);
    };
    Ok((stage_buf, ring))
}

/*
 * Map `bytes` (whole pages) uncached and below 4 GiB, as rtw88 holds this
 * chip's rings and buffers (dma_alloc_coherent under a 32-bit mask): the
 * descriptors carry 32-bit addresses, so a region above 4 GiB would be
 * truncated and the card would read or write other memory. The broker
 * places a DMA32 map below 4 GiB or refuses it; a grant that still ends
 * above is given back and refused here too.
 */
pub(crate) fn map_dma32(device_id: u64, claim_epoch: u64, bytes: u64) -> Option<Grant> {
    let flags = MK_DMA_MAP_COHERENT | MK_DMA_MAP_DMA32;
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, claim_epoch, bytes, flags, &mut out) < 0 {
        return None;
    }
    if !reachable(out.device_addr, out.length as usize) {
        let _ = mk_dma_unmap(out.grant_id);
        status::line(b"[rtl8821ce] DMA map above 4 GiB refused\n");
        return None;
    }
    // SAFETY: the broker mapped `length` coherent bytes at `user_va` for this
    // claim, reached by the card at `device_addr`; the mapping lasts as long
    // as the claim, which outlives every grant the driver holds.
    Some(unsafe { Grant::new(out.user_va, out.device_addr, out.length as usize) })
}
