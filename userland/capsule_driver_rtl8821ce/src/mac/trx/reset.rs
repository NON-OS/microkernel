// NONOS Operating System (AGPL-3.0-or-later)

//! The ring pointers and DMA interface reset, and the read-modify-write helpers.

use super::regs::{
    CLR_H2CQ_HOST_IDX, CLR_H2CQ_HW_IDX, REG_H2CQ_CSR, REG_PCI_CTRL, REG_TXBD_RWPTR_CLR,
    RST_TRXDMA_INTF, RX_TAG_EN,
};
use crate::regs::Mmio;

/// Reset the transmit and receive ring pointers and the DMA interface once the
/// ring addresses are programmed. This is rtw88 `rtw_pci_reset_buf_desc` (its
/// pointer-clear tail) plus `rtw_pci_dma_reset`: clear the ring read/write
/// pointers, clear the H2C queue host and hardware indices, then reset the TRX
/// DMA interface with the receive tag enabled. Without the interface reset the
/// card never advances its receive write index, so no frame is ever delivered.
/// Call after the transmit and receive rings have their addresses and lengths
/// set.
pub fn reset_trx_dma<M: Mmio>(mmio: &M) {
    mmio.write32(REG_TXBD_RWPTR_CLR, 0xFFFF_FFFF);
    let csr = mmio.read32(REG_H2CQ_CSR);
    mmio.write32(REG_H2CQ_CSR, csr | CLR_H2CQ_HOST_IDX | CLR_H2CQ_HW_IDX);
    let ctrl = mmio.read32(REG_PCI_CTRL);
    mmio.write32(REG_PCI_CTRL, ctrl | RST_TRXDMA_INTF | RX_TAG_EN);
}

pub(super) fn set8<M: Mmio>(mmio: &M, off: usize, bit: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v | bit);
}

pub(super) fn set32<M: Mmio>(mmio: &M, off: usize, bit: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, v | bit);
}

// Replace only the bits outside `keep` with `set`, leaving the kept bits as the
// hardware brought them up.
pub(super) fn rmw8<M: Mmio>(mmio: &M, off: usize, keep: u8, set: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, (v & keep) | set);
}

pub(super) fn rmw32<M: Mmio>(mmio: &M, off: usize, keep: u32, set: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, (v & keep) | set);
}
