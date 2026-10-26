// NONOS Operating System (AGPL-3.0-or-later)

//! The reserved pages into the chip's packet buffer through the beacon
//! queue, at the driver's first reserved page, then the stand-in beacon again
//! so page 0 carries a descriptor of its own (`rtw_fw_download_rsvd_page`).

use crate::fw::dma::DmaMem;
use crate::fw::rsvd::stage_at;
use crate::mac::RSVD_BOUNDARY;
use crate::regs::Mmio;

/// True once both landed.
pub fn download<M: Mmio, B: DmaMem>(mmio: &M, stage: &B, ring: &B, pages: &[u8]) -> bool {
    stage_at(mmio, stage, ring, RSVD_BOUNDARY, pages)
        && stage_at(mmio, stage, ring, RSVD_BOUNDARY, &pages[..1])
}
