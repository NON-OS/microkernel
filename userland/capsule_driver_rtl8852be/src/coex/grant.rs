// NONOS Operating System (AGPL-3.0-or-later)

//! The antenna given to Wi-Fi alone: `_set_ant_v0` for `BTC_ANT_WONLY`
//! (Linux v6.12 rtw89 `coex.c`). Both paths get WL granted by software
//! (`_set_gnt` SW_HI) and BT denied by software (SW_LO), written as one
//! word by `rtw89_mac_cfg_gnt` (`mac.c`); the control path goes to WL
//! (`rtw89_mac_cfg_ctrl_path`); the BT polluter table is cleared to "none"
//! (`_set_bt_plut`, `rtw89_mac_cfg_plt_ax`). No Bluetooth stack runs on
//! NONOS, so this is the arbitration rtw89 itself picks when BT is off.

use super::lte::{self, LteBusy, R_AX_LTE_SW_CFG_1};
use crate::regs::{Mmio, Rw};

const WL_S0: u32 = 0x2000 | 0x200; // RFC and BB SW_VAL
const WL_S0_CTRL: u32 = 0x1000 | 0x100;
const BT_S0_CTRL: u32 = 0x4000 | 0x400;
const WL_S1: u32 = 0x2000_0000 | 0x200_0000;
const WL_S1_CTRL: u32 = 0x1000_0000 | 0x100_0000;
const BT_S1_CTRL: u32 = 0x4000_0000 | 0x400_0000;
/// WL forced high and BT forced low on both paths.
pub const GNT_WL_ONLY: u32 = WL_S0 | WL_S0_CTRL | BT_S0_CTRL | WL_S1 | WL_S1_CTRL | BT_S1_CTRL;

pub const R_AX_SYS_SDIO_CTRL: u32 = 0x70;
const CTRL_BY_WL: u8 = 1 << 2;
pub const R_AX_BT_PLT: u32 = 0xc67c;
const B_AX_PLT_EN: u16 = 0x100;

pub fn wl_only(r: &impl Mmio) -> Result<(), LteBusy> {
    lte::write(r, R_AX_LTE_SW_CFG_1, GNT_WL_ONLY)?;
    r.set8(R_AX_SYS_SDIO_CTRL + 3, CTRL_BY_WL);
    r.write16(R_AX_BT_PLT, B_AX_PLT_EN);
    Ok(())
}
