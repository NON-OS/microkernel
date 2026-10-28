// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_pci_ops_mac_post_init_ax` (Linux v6.12 rtw89 `pci.c`) after the
//! MAC is up: the latency tolerance reporting (`rtw89_pci_ltr_set`, which
//! refuses a register that reads as an error pattern), the 8-byte address
//! info again, every transmit channel let run, and PCIe IO released.

use super::mode::addr_info_8b;
use super::regs::*;
use super::PcieFail;
use crate::regs::{Mmio, Rw};

const LTR_SPC_500US: u32 = 2;
const LTR_IDLE_TIMER_3_2MS: u32 = 7;

/// `rtw89_pci_ltr_is_err_reg_val`.
pub const fn ltr_bad(v: u32) -> bool {
    v == 0xffff_ffff || v == 0xeaea_eaea
}

pub fn post_init(r: &impl Mmio) -> Result<(), PcieFail> {
    let regs = [R_AX_LTR_CTRL_0, R_AX_LTR_CTRL_1, R_AX_LTR_IDLE_LATENCY, R_AX_LTR_ACTIVE_LATENCY];
    if regs.iter().any(|&a| ltr_bad(r.read32(a))) {
        return Err(PcieFail::Ltr);
    }
    r.set32(R_AX_LTR_CTRL_0, B_AX_LTR_EN_BITS);
    r.mask32(R_AX_LTR_CTRL_0, B_AX_LTR_SPACE_IDX_MASK, LTR_SPC_500US);
    r.mask32(R_AX_LTR_CTRL_0, B_AX_LTR_IDLE_TIMER_IDX_MASK, LTR_IDLE_TIMER_3_2MS);
    r.mask32(R_AX_LTR_CTRL_1, B_AX_LTR_RX0_TH_MASK, 0x28);
    r.mask32(R_AX_LTR_CTRL_1, B_AX_LTR_RX1_TH_MASK, 0x28);
    r.write32(R_AX_LTR_IDLE_LATENCY, 0x9003_9003);
    r.write32(R_AX_LTR_ACTIVE_LATENCY, 0x880b_880b);
    addr_info_8b(r);
    r.clr32(R_AX_PCIE_DMA_STOP1, B_AX_TX_STOP1_MASK_V1);
    r.clr32(R_AX_PCIE_DMA_STOP1, B_AX_STOP_WPDMA | B_AX_STOP_PCIEIO);
    Ok(())
}
