// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_dmac_pre_init` (Linux v6.12 rtw89 `mac.c`): the DMAC's HCI
//! functions and dispatcher clock on (`rtw89_mac_hci_func_en_ax`,
//! `rtw89_mac_dmac_func_pre_en_ax`), the download memory plan with the
//! service plan's CPU minimum, and flow control for the command queue only.
//! Before it, `rtw89_mac_partial_init` turns the HCI DMA on
//! (`rtw89_mac_ctrl_hci_dma_trx`).

use super::{mac_on, MacFail};
use crate::dle::{dle_init, hfc_init, quota};
use crate::regs::{Mmio, Rw};

pub const R_AX_HCI_FUNC_EN: u32 = 0x8380;
pub const HCI_TXRX_DMA_EN: u32 = 0x3;
pub const R_AX_DMAC_FUNC_EN: u32 = 0x8400;
pub const R_AX_DMAC_CLK_EN: u32 = 0x8404;
/// MAC_FUNC_EN, DMAC_FUNC_EN, DISPATCHER_EN, PKT_BUF_EN.
pub const HCI_FUNC: u32 = 0x6044_0000;
pub const DISPATCHER_CLK_EN: u32 = 0x4_0000;

pub fn hci_dma_on(r: &impl Mmio) {
    r.set32(R_AX_HCI_FUNC_EN, HCI_TXRX_DMA_EN);
}

pub fn dmac_pre_init(r: &impl Mmio) -> Result<(), MacFail> {
    r.write32(R_AX_DMAC_FUNC_EN, HCI_FUNC);
    r.write32(R_AX_DMAC_CLK_EN, DISPATCHER_CLK_EN);
    mac_on(r)?;
    dle_init(r, &quota::DLFW, Some(quota::SCC.wde_min[1])).map_err(MacFail::Dle)?;
    mac_on(r)?;
    hfc_init(r, false, true).map_err(MacFail::Hfc)
}
