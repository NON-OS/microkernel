// NONOS Operating System (AGPL-3.0-or-later)

//! After the second download: `__rtw8852bx_mac_enable_bb_rf` (Linux v6.12
//! rtw89 `rtw8852b_common.c`) and `sys_init_ax` with `dmac_func_en_ax`,
//! `cmac_func_en_ax` and `chip_func_en_ax` (`mac.c`). The DMAC enables are
//! written whole, dropping the data link engine until the TRX init sets
//! its service plan.

use super::MacFail;
use crate::pwr::xtal::{self, XTAL_SI_WL_RFC_S0, XTAL_SI_WL_RFC_S1};
use crate::regs::{Mmio, Rw};

pub const R_AX_SYS_FUNC_EN: u32 = 0x2;
pub const FEN_BB: u8 = 0x3;
pub const R_AX_SPS_DIG_ON_CTRL0: u32 = 0x200;
pub const B_AX_REG_ZCDC_H_MASK: u32 = 0x6_0000;
pub const B_AX_OCP_L1_MASK: u32 = 0xe000;
pub const R_AX_WLRF_CTRL: u32 = 0x2f0;
pub const B_AX_AFC_AFEDIG: u32 = 0x2_0000;
pub const R_AX_PHYREG_SET: u32 = 0x8040;
pub const PHYREG_SET_XYN_CYCLE: u8 = 0xe;
pub const R_AX_DMAC_FUNC_EN: u32 = 0x8400;
pub const R_AX_DMAC_CLK_EN: u32 = 0x8404;
pub const R_AX_CMAC_FUNC_EN: u32 = 0xc000;
pub const R_AX_CK_EN: u32 = 0xc004;
/// The DMAC functions `dmac_func_en_ax` writes for every chip but the 8852C.
pub const DMAC_FUNC: u32 = 0xfb7d_0000;
/// Its clocks, `B_AX_*_CLK_EN` (no AXIDMA on the 8852B).
pub const DMAC_CLK: u32 = 0x0b1f_0000;
pub const CMAC_CK: u32 = 0x4000_003f;
pub const CMAC_FUNC: u32 = 0xf000_003f;

pub fn enable_bb_rf(r: &impl Mmio) -> Result<(), MacFail> {
    r.set8(R_AX_SYS_FUNC_EN, FEN_BB);
    r.mask32(R_AX_SPS_DIG_ON_CTRL0, B_AX_REG_ZCDC_H_MASK, 0x1);
    r.set32(R_AX_WLRF_CTRL, B_AX_AFC_AFEDIG);
    r.clr32(R_AX_WLRF_CTRL, B_AX_AFC_AFEDIG);
    r.set32(R_AX_WLRF_CTRL, B_AX_AFC_AFEDIG);
    let x = |off| xtal::write(r, off, 0xc7, 0xff).map_err(|_| MacFail::Step(0));
    x(XTAL_SI_WL_RFC_S0)?;
    x(XTAL_SI_WL_RFC_S1)?;
    r.write8(R_AX_PHYREG_SET, PHYREG_SET_XYN_CYCLE);
    Ok(())
}

/// `__rtw8852bx_mac_disable_bb_rf`; `rtw89_chip_reset_bb_rf` runs it and
/// then `enable_bb_rf` again before the BB tables.
pub fn disable_bb_rf(r: &impl Mmio) -> Result<(), MacFail> {
    r.clr32(R_AX_WLRF_CTRL, B_AX_AFC_AFEDIG);
    r.clr8(R_AX_SYS_FUNC_EN, FEN_BB);
    for off in [XTAL_SI_WL_RFC_S0, XTAL_SI_WL_RFC_S1] {
        let v = xtal::read(r, off).map_err(|_| MacFail::Step(0))?;
        xtal::write(r, off, v & !0x7, 0xff).map_err(|_| MacFail::Step(0))?;
    }
    Ok(())
}

pub fn sys_init(r: &impl Mmio) {
    r.write32(R_AX_DMAC_FUNC_EN, DMAC_FUNC);
    r.write32(R_AX_DMAC_CLK_EN, DMAC_CLK);
    r.set32(R_AX_CK_EN, CMAC_CK);
    r.set32(R_AX_CMAC_FUNC_EN, CMAC_FUNC);
    r.set32(R_AX_SPS_DIG_ON_CTRL0, B_AX_OCP_L1_MASK);
}
