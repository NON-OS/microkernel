// NONOS Operating System (AGPL-3.0-or-later)

//! The four DMA regions the data path needs, mapped through the broker.

use nonos_libc::{MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use crate::fw::dma::Grant;
use crate::ring::TX_BUF_DESC_SIZE;
use crate::rx::regs::RX_BUF_DESC_SIZE;
use crate::rx::ring::{RX_BUF_STRIDE, RX_DESC_COUNT};
use crate::tx::ring::{TX_BUF_STRIDE, TX_DESC_COUNT};

/// Map the four DMA regions the data path needs, each rounded up to whole pages.
type Rings = (Grant, Grant, Grant, Grant);

pub(super) fn map_rings(device_id: u64, claim_epoch: u64) -> Option<Rings> {
    let tx_ring = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_DESC_SIZE)?;
    let tx_buffers = map(device_id, claim_epoch, TX_DESC_COUNT as usize * TX_BUF_STRIDE)?;
    let rx_ring = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_DESC_SIZE)?;
    let rx_buffers = map(device_id, claim_epoch, RX_DESC_COUNT as usize * RX_BUF_STRIDE)?;
    Some((tx_ring, tx_buffers, rx_ring, rx_buffers))
}

/*
 * Round a byte count up to whole pages and map it through the broker,
 * uncached and below 4 GiB, as rtw88 holds this chip's rings and buffers
 * (dma_alloc_coherent under a 32-bit mask): the buffer descriptors carry
 * 32-bit addresses, and on a machine whose remapping unit has no snoop
 * control a cached ring or buffer can show the card or the driver bytes the
 * other side already replaced.
 */
pub(super) fn map(device_id: u64, claim_epoch: u64, bytes: usize) -> Option<Grant> {
    let pages = ((bytes + 0xFFF) & !0xFFF) as u64;
    let flags = MK_DMA_MAP_COHERENT | MK_DMA_MAP_DMA32;
    crate::fwload::map_dma_with(device_id, claim_epoch, pages, flags)
}
