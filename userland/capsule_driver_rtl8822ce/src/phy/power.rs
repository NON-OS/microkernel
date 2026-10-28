// NONOS Operating System (AGPL-3.0-or-later)

//! The prologue of rtw88 `rtw8822c_phy_set_param` (rtw8822c.c:1874-1885):
//! the baseband and RF domains powered, low-rate DPD off; and its
//! `rtw8822c_header_file_init` and `rtw8822c_bb_reset`. Register facts from
//! rtw8822c.h and reg.h (Linux v6.12).

use super::bits::{clr32, set32, set8, write32_mask};
use crate::regs::Mmio;

const REG_SYS_FUNC_EN: usize = 0x0002;
/// `BIT_FEN_BB_GLB_RST | BIT_FEN_BB_RSTB`.
const FEN_BB: u8 = (1 << 1) | (1 << 0);
const REG_RF_CTRL: usize = 0x001F;
/// `BIT_RF_EN | BIT_RF_RSTB | BIT_RF_SDM_RSTB`.
const RF_EN_RSTB: u8 = (1 << 0) | (1 << 1) | (1 << 2);
const REG_WLRF1: usize = 0x00EC;
/// `BIT_WLRF1_BBRF_EN`.
const WLRF1_BBRF_EN: u32 = 0x0700_0000;
/// `REG_DIS_DPD`, `DIS_DPD_MASK` = `DIS_DPD_RATEALL` = GENMASK(9, 0).
pub(super) const REG_DIS_DPD: usize = 0x0A70;
pub(super) const DIS_DPD_ALL: u32 = 0x3FF;

/// `REG_3WIRE`, `REG_3WIRE2`: the paths' three-wire interfaces.
const REG_3WIRE: [usize; 2] = [0x180C, 0x410C];
/// `BIT_3WIRE_TX_EN | BIT_3WIRE_RX_EN`, and `BIT_3WIRE_PI_ON`.
const WIRE_TRX_EN: u32 = (1 << 0) | (1 << 1);
const WIRE_PI_ON: u32 = 1 << 28;
/// `REG_ENCCK` and `BIT_CCK_OFDM_BLK_EN`.
const REG_ENCCK: usize = 0x1C3C;
const CCK_OFDM_BLK_EN: u32 = (1 << 0) | (1 << 1);

/// Power the BB and RF domains, and turn DPD off for every rate.
pub fn power_on<M: Mmio>(mmio: &M) {
    set8(mmio, REG_SYS_FUNC_EN, FEN_BB);
    set8(mmio, REG_RF_CTRL, RF_EN_RSTB);
    set32(mmio, REG_WLRF1, WLRF1_BBRF_EN);
    write32_mask(mmio, REG_DIS_DPD, DIS_DPD_ALL, DIS_DPD_ALL);
}

/// `rtw8822c_header_file_init`: both three-wire interfaces on, and the CCK
/// and OFDM blocks off before the tables (`pre`) and on after them.
pub fn header_init<M: Mmio>(mmio: &M, pre: bool) {
    for wire in REG_3WIRE {
        set32(mmio, wire, WIRE_TRX_EN);
        set32(mmio, wire, WIRE_PI_ON);
    }
    if pre {
        clr32(mmio, REG_ENCCK, CCK_OFDM_BLK_EN);
    } else {
        set32(mmio, REG_ENCCK, CCK_OFDM_BLK_EN);
    }
}

/// `rtw8822c_bb_reset`: the baseband reset released, asserted, released.
pub fn bb_reset<M: Mmio>(mmio: &M) {
    let v = mmio.read16(REG_SYS_FUNC_EN);
    mmio.write16(REG_SYS_FUNC_EN, v | 1);
    let v = mmio.read16(REG_SYS_FUNC_EN);
    mmio.write16(REG_SYS_FUNC_EN, v & !1);
    let v = mmio.read16(REG_SYS_FUNC_EN);
    mmio.write16(REG_SYS_FUNC_EN, v | 1);
}
