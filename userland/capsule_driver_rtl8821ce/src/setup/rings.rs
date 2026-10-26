// NONOS Operating System (AGPL-3.0-or-later)

//! The four DMA regions the data path needs, mapped through the broker.

use crate::fw::dma::Grant;
use crate::ring::TX_BUF_DESC_SIZE;
use crate::rx::regs::RX_BUF_DESC_SIZE;
use crate::rx::ring::{RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::tx::ring::{TX_BUF_STRIDE, TX_DESC_COUNT};

/// Map the four DMA regions the data path needs, each rounded up to whole pages.
pub type Rings = (Grant, Grant, Grant, Grant);

pub fn map_rings(device_id: u64, claim_epoch: u64) -> Option<Rings> {
    let tx_ring = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_DESC_SIZE)?;
    let tx_buffers = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_STRIDE)?;
    let rx_ring = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_DESC_SIZE)?;
    let rx_buffers = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_STRIDE)?;
    Some((tx_ring, tx_buffers, rx_ring, rx_buffers))
}

// Round a byte count up to whole pages and map it below 4 GiB.
fn map(device_id: u64, claim_epoch: u64, bytes: usize) -> Option<Grant> {
    let pages = ((bytes + 0xFFF) & !0xFFF) as u64;
    crate::fwload::map_dma32(device_id, claim_epoch, pages)
}
