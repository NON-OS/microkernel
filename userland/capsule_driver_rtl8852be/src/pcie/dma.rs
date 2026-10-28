// NONOS Operating System (AGPL-3.0-or-later)

//! The second half of `rtw89_pci_ops_mac_pre_init_ax` (Linux v6.12 rtw89
//! `pci.c`): every DMA stopped and seen idle, the ring pointers cleared,
//! the mode set, the rings programmed, the BD RAM reset, and only the
//! firmware command channel let run, as the download needs it alone.

use super::regs::*;
use super::{mode, PcieFail};
use crate::dma::DmaMem;
use crate::regs::{Mmio, Rw};
use crate::ring::Rings;
use crate::wait::until_us;

pub fn dma_setup<M: DmaMem>(r: &impl Mmio, rings: &mut Rings<M>) -> Result<(), PcieFail> {
    r.set32(R_AX_PCIE_DMA_STOP1, B_AX_STOP_WPDMA);
    dma_all(r, false);
    let idle = |reg, bits| until_us(100, || r.read32(reg) & bits == 0);
    if !idle(R_AX_PCIE_DMA_BUSY1, DMA_BUSY1_CHECK_V1) {
        return Err(PcieFail::TxBusy);
    }
    if !idle(R_AX_PCIE_DMA_BUSY1, RX_BUSY) {
        return Err(PcieFail::RxBusy);
    }
    clear_idx(r);
    mode::mode_op(r);
    rings.program(r);
    r.set32(R_AX_PCIE_INIT_CFG1, B_AX_RST_BDRAM);
    if !until_us(100, || r.read32(R_AX_PCIE_INIT_CFG1) & B_AX_RST_BDRAM == 0) {
        return Err(PcieFail::BdRam);
    }
    r.set32(R_AX_PCIE_DMA_STOP1, B_AX_TX_STOP1_MASK_V1);
    r.clr32(R_AX_PCIE_DMA_STOP1, B_AX_STOP_CH12);
    dma_all(r, true);
    Ok(())
}

/// `rtw89_pci_ctrl_dma_all`: PCIe IO and the host DMA engines together.
pub fn dma_all(r: &impl Mmio, on: bool) {
    if on {
        r.clr32(R_AX_PCIE_DMA_STOP1, B_AX_STOP_PCIEIO);
        r.set32(R_AX_PCIE_INIT_CFG1, B_AX_RXHCI_EN | B_AX_TXHCI_EN);
    } else {
        r.set32(R_AX_PCIE_DMA_STOP1, B_AX_STOP_PCIEIO);
        r.clr32(R_AX_PCIE_INIT_CFG1, B_AX_RXHCI_EN | B_AX_TXHCI_EN);
    }
}

/// `rtw89_pci_clr_idx_all_ax` on the 8852B.
pub fn clear_idx(r: &impl Mmio) {
    r.set32(R_AX_TXBD_RWPTR_CLR1, TX_CLR_8852B);
    r.set32(R_AX_RXBD_RWPTR_CLR, RX_CLR);
}
