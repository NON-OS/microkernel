// NONOS Operating System (AGPL-3.0-or-later)

//! `cmac_init_ax`'s remaining steps (Linux v6.12 rtw89 `mac.c`) for CMAC 0:
//! `tmac_init_ax`, `trxptcl_init_ax` (the 8852B's SIFS, `rtw8852b_rrsr_cfgs`),
//! `rmac_init_ax` (the BA CAM reset, receive deadlock guard, one channel,
//! the largest MPDU the receive quota holds), `cmac_com_init_ax`,
//! `ptcl_init_ax` and `cmac_dma_init_ax`.

use super::{cmac_on, MacFail};
use crate::regs::{Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_TCR0: u32 = 0xca00;
pub const R_AX_TXD_FIFO_CTRL: u32 = 0xca1c;
pub const R_AX_MAC_LOOPBACK: u32 = 0xcc20;
pub const R_AX_TRXPTCL_RESP_0: u32 = 0xcc04;
pub const R_AX_TRXPTCL_RRSR_CTL_0: u32 = 0xcc08;
pub const R_AX_RXTRIG_TEST_USER_2: u32 = 0xccb0;
pub const R_AX_RESPBA_CAM_CTRL: u32 = 0xce3c;
pub const R_AX_DLK_PROTECT_CTL: u32 = 0xce02;
pub const R_AX_RCR: u32 = 0xce00;
pub const R_AX_RX_FLTR_OPT: u32 = 0xce20;
pub const R_AX_PLCP_HDR_FLTR: u32 = 0xce04;
pub const R_AX_TX_SUB_CARRIER_VALUE: u32 = 0xc088;
pub const R_AX_PTCL_RRSR1: u32 = 0xc090;
pub const R_AX_PTCL_COMMON_SETTING_0: u32 = 0xc600;
pub const R_AX_AGG_LEN_VHT_0: u32 = 0xc618;
pub const R_AX_SIFS_SETTING: u32 = 0xc624;
pub const R_AX_PTCLRPT_FULL_HDL: u32 = 0xc660;
pub const R_AX_PTCL_FSM_MON: u32 = 0xc6e8;
pub const R_AX_RXDMA_CTRL_0: u32 = 0xc804;
/// The receive MPDU limit in 512-byte units: the SCC plan's CMAC0 DMA
/// quota (89 pages of 128 bytes) as `rmac_init_ax` computes it.
pub const RX_MAX_LEN_UNITS: u32 = 89 * 128 / 512;

pub fn cmac_b(r: &impl Mmio) -> Result<(), MacFail> {
    cmac_on(r)?;
    r.clr32(R_AX_MAC_LOOPBACK, 0x1);
    r.mask32(R_AX_TCR0, 0x7f_0000, 0x6);
    r.mask32(R_AX_TXD_FIFO_CTRL, 0xf000, 0x7);
    r.mask32(R_AX_TXD_FIFO_CTRL, 0xf00, 0x7);
    let resp = r.read32(R_AX_TRXPTCL_RESP_0) & !0xffff;
    r.write32(R_AX_TRXPTCL_RESP_0, resp | 0x0a | 0x11 << 8);
    r.set32(R_AX_RXTRIG_TEST_USER_2, 0x10_0000);
    r.mask32(R_AX_TRXPTCL_RRSR_CTL_0, 0x200, 0);
    r.mask32(R_AX_TRXPTCL_RRSR_CTL_0, 0xc00, 2);
    r.mask32(R_AX_RESPBA_CAM_CTRL, 0x3, 0x2);
    // rtw89 only warns when the BA CAM reset does not finish (`rst_bacam`).
    if !until_us(1000, || r.read32(R_AX_RESPBA_CAM_CTRL) & 0x3 == 0) {
        crate::status::line(b"[rtl8852be] mac: BA CAM reset did not finish\n");
    }
    r.set8(R_AX_RESPBA_CAM_CTRL, 0x4);
    let dlk = (r.read16(R_AX_DLK_PROTECT_CTL) & !0xfff0) | 15 << 4 | 32 << 8;
    r.write16(R_AX_DLK_PROTECT_CTL, dlk);
    r.mask8(R_AX_RCR, 0xf, 0x1);
    r.mask32(R_AX_RX_FLTR_OPT, 0x3f_0000, RX_MAX_LEN_UNITS);
    r.clr8(R_AX_PLCP_HDR_FLTR, 0x10);
    let sc = r.read32(R_AX_TX_SUB_CARRIER_VALUE) & !0xfff;
    r.write32(R_AX_TX_SUB_CARRIER_VALUE, sc);
    r.mask32(R_AX_PTCL_RRSR1, 0xf00, 0x3);
    ptcl_and_dma(r);
    Ok(())
}

fn ptcl_and_dma(r: &impl Mmio) {
    let sifs = r.read32(R_AX_SIFS_SETTING) & !0xfffc_0000;
    r.write32(R_AX_SIFS_SETTING, sifs | 0x4 << 24 | 0x1 << 18 | 0x1_0000);
    let mon = (r.read32(R_AX_PTCL_FSM_MON) & !0x7f) | 0x3f;
    r.write32(R_AX_PTCL_FSM_MON, mon);
    r.set8(R_AX_PTCL_COMMON_SETTING_0, 0x3);
    r.clr8(R_AX_PTCL_COMMON_SETTING_0, 0x1c);
    r.mask8(R_AX_PTCLRPT_FULL_HDL, 0x30, 0x1);
    r.mask32(R_AX_AGG_LEN_VHT_0, 0xf_ffff, 0x3_ff80);
    r.clr8(R_AX_RXDMA_CTRL_0, 0x3f);
}
