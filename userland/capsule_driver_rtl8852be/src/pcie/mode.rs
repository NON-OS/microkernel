// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_pci_mode_op` (Linux v6.12 rtw89 `pci.c`) with the 8852B's PCI
//! info (`rtw8852be.c`, `rtw8852b_pci_info`): truncated descriptors (which
//! need no register on this chip), one buffer per received packet, 2048-
//! byte transmit and 128-byte receive bursts, eight DMA tags, a 256 ns WD
//! fetch interval, and the 8-byte address info.

use super::regs::*;
use crate::regs::{Mmio, Rw};

pub const TX_BURST_2048B: u32 = 7;
pub const RX_BURST_128B: u32 = 3;
pub const TAG_NUM_8: u32 = 7;
pub const WD_DMA_INTVL_256NS: u32 = 1;

pub fn mode_op(r: &impl Mmio) {
    r.clr32(R_AX_PCIE_INIT_CFG1, B_AX_RXBD_MODE);
    r.mask32(R_AX_PCIE_INIT_CFG1, B_AX_PCIE_MAX_TXDMA_MASK, TX_BURST_2048B);
    r.mask32(R_AX_PCIE_INIT_CFG1, B_AX_PCIE_MAX_RXDMA_MASK, RX_BURST_128B);
    let v = r.read32(R_AX_PCIE_INIT_CFG1) | B_AX_LATENCY_CONTROL;
    r.write32(R_AX_PCIE_INIT_CFG1, v);
    r.mask32(R_AX_PCIE_EXP_CTRL, B_AX_MAX_TAG_NUM, TAG_NUM_8);
    r.mask32(R_AX_PCIE_INIT_CFG2, B_AX_WD_ITVL_IDLE, WD_DMA_INTVL_256NS);
    r.mask32(R_AX_PCIE_INIT_CFG2, B_AX_WD_ITVL_ACT, WD_DMA_INTVL_256NS);
    addr_info_8b(r);
}

/// The 8-byte address info: `mode_op` for truncated descriptors, and again
/// in `rtw89_pci_ops_mac_post_init_ax`.
pub fn addr_info_8b(r: &impl Mmio) {
    r.set32(R_AX_TX_ADDRESS_INFO_MODE_SETTING, B_AX_HOST_ADDR_INFO_8B_SEL);
    r.clr32(R_AX_PKTIN_SETTING, B_AX_WD_ADDR_INFO_LENGTH);
}
