// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_mac_dle_init` (Linux v6.12 rtw89 `mac.c`) with `dle_mix_cfg_ax`,
//! `wde_quota_cfg_ax` and `ple_quota_cfg_ax`: the engines off, their clocks
//! on, the page buffers split, every queue's minimum and maximum set, the
//! engines on, and both seen ready within 2 ms.

use super::quota::{ple_bound, Plan, Ple, Wde};
use crate::regs::{field, Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_DMAC_FUNC_EN: u32 = 0x8400;
pub const R_AX_DMAC_CLK_EN: u32 = 0x8404;
pub const DLE_EN: u32 = 0x0480_0000;
pub const DLE_CLK_EN: u32 = 0x0480_0000;
pub const R_AX_WDE_PKTBUF_CFG: u32 = 0x8c08;
pub const R_AX_PLE_PKTBUF_CFG: u32 = 0x9008;
const PAGE_SEL: u32 = 0x3;
const START_BOUND: u32 = 0x3f00;
const FREE_PAGE_NUM: u32 = 0x1fff_0000;
/// `S_AX_WDE_PAGE_SEL_64`, `S_AX_PLE_PAGE_SEL_128`.
const WDE_SEL_64: u32 = 0;
const PLE_SEL_128: u32 = 1;
pub const R_AX_WDE_QTA0_CFG: u32 = 0x8c40;
pub const R_AX_PLE_QTA0_CFG: u32 = 0x9040;
pub const R_AX_WDE_INI_STATUS: u32 = 0x8d00;
pub const R_AX_PLE_INI_STATUS: u32 = 0x9100;
pub const MGN_INI_RDY: u32 = 0x3;
/// The WDE quotas' register indices: hif 0, wcpu 1, pkt_in 3, cpu_io 4.
const WDE_IDX: [u32; 4] = [0, 1, 3, 4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DleFail {
    WdeNotReady,
    PleNotReady,
}

/// Lay `plan` out; `wcpu_min`, when given, replaces the WDE CPU minimum (the
/// download plan takes the SCC plan's, `ext_wde_min_qt_wcpu`).
pub fn dle_init(r: &impl Mmio, plan: &Plan, wcpu_min: Option<u16>) -> Result<(), DleFail> {
    r.clr32(R_AX_DMAC_FUNC_EN, DLE_EN);
    r.set32(R_AX_DMAC_CLK_EN, DLE_CLK_EN);
    let wde = r.read32(R_AX_WDE_PKTBUF_CFG) & !(PAGE_SEL | START_BOUND | FREE_PAGE_NUM);
    r.write32(R_AX_WDE_PKTBUF_CFG, wde | WDE_SEL_64 | field(FREE_PAGE_NUM, plan.wde.1));
    let ple = r.read32(R_AX_PLE_PKTBUF_CFG) & !(PAGE_SEL | START_BOUND | FREE_PAGE_NUM);
    let bound = field(START_BOUND, ple_bound(plan));
    r.write32(R_AX_PLE_PKTBUF_CFG, ple | PLE_SEL_128 | bound | field(FREE_PAGE_NUM, plan.ple.1));
    let mut wde_min: Wde = plan.wde_min;
    if let Some(m) = wcpu_min {
        wde_min[1] = m;
    }
    quotas(r, R_AX_WDE_QTA0_CFG, &WDE_IDX, &wde_min, &plan.wde_max);
    let idx: [u32; 11] = core::array::from_fn(|i| i as u32);
    quotas(r, R_AX_PLE_QTA0_CFG, &idx, &plan.ple_min, &plan.ple_max as &Ple);
    r.set32(R_AX_DMAC_FUNC_EN, DLE_EN);
    let ready = |reg| until_us(2000, || r.read32(reg) & MGN_INI_RDY == MGN_INI_RDY);
    if !ready(R_AX_WDE_INI_STATUS) {
        return Err(DleFail::WdeNotReady);
    }
    if !ready(R_AX_PLE_INI_STATUS) {
        return Err(DleFail::PleNotReady);
    }
    Ok(())
}

/// `SET_QUOTA_VAL`: min in 11:0, max in 27:16, one register per queue.
fn quotas(r: &impl Mmio, base: u32, idx: &[u32], min: &[u16], max: &[u16]) {
    for ((i, lo), hi) in idx.iter().zip(min).zip(max) {
        let v = field(0xfff, *lo as u32) | field(0x0fff_0000, *hi as u32);
        r.write32(base + i * 4, v);
    }
}
