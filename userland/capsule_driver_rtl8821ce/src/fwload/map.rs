// NONOS Operating System (AGPL-3.0-or-later)

//! The two DMA regions the reserved-page path needs, mapped through the broker.

use nonos_libc::{mk_dma_map, DmaMapOut};

use super::FwLoadError;
use crate::fw::dma::{DmaMem, Grant};
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
    let Some(stage_buf) = map_dma(device_id, claim_epoch, STAGE_BYTES) else {
        status::line(b"[rtl8821ce] fw: staging DMA map failed\n");
        return Err(FwLoadError::Dma);
    };
    let Some(ring) = map_dma(device_id, claim_epoch, RING_BYTES) else {
        status::line(b"[rtl8821ce] fw: ring DMA map failed\n");
        return Err(FwLoadError::Dma);
    };

    // The card addresses staged pages with a 32-bit descriptor field, so a
    // buffer above 4GB would be silently truncated and the beacon DMA would read
    // the wrong memory. Report the high half of each address: a non-zero value
    // here is the reason a staged page never validates on a large-memory machine.
    let stage_hi = (stage_buf.device_addr() >> 32) as u32;
    let ring_hi = (ring.device_addr() >> 32) as u32;
    if stage_hi != 0 || ring_hi != 0 {
        status::line(b"[rtl8821ce] fw: DMA buffer above 4GB (stage hi=");
        status::number(stage_hi);
        status::line(b" ring hi=");
        status::number(ring_hi);
        status::line(b"), 32-bit descriptor cannot address it\n");
    }
    Ok((stage_buf, ring))
}

/// Map one DMA region and wrap it as a coherent buffer with its device address.
/// Shared with the serving stage, which maps the transmit and receive rings the
/// same way.
pub(crate) fn map_dma(device_id: u64, claim_epoch: u64, bytes: u64) -> Option<Grant> {
    map_dma_with(device_id, claim_epoch, bytes, 0)
}

/// `map_dma` with the broker's map flags (`MK_DMA_MAP_*`).
pub(crate) fn map_dma_with(
    device_id: u64,
    claim_epoch: u64,
    bytes: u64,
    flags: u32,
) -> Option<Grant> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, claim_epoch, bytes, flags, &mut out) < 0 {
        return None;
    }
    // SAFETY: the broker mapped `length` coherent bytes at `user_va` for this
    // claim, reached by the card at `device_addr`; the mapping lasts as long
    // as the claim, which outlives every grant the driver holds.
    Some(unsafe { Grant::new(out.user_va, out.device_addr, out.length as usize) })
}
