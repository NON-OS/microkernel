// NONOS Operating System (AGPL-3.0-or-later)

//! rtw88's Wi-Fi-only coexistence init for the 8822C (coex.c
//! `__rtw_coex_init_hw_config` with wifi_only, `COEX_SET_ANT_WONLY` at
//! coex.c:1316): LTE coex off in the Wi-Fi side (`rtw8822c_coex_cfg_rfe_type`),
//! the chip's coex init, Wi-Fi's TX responses and beacons at high priority,
//! Bluetooth's grant low and Wi-Fi's high in software, Wi-Fi the owner of the
//! path control, and the scoreboard saying Wi-Fi is active and on.

use super::indirect::write_field;
use super::init::{cfg_init, gnt_debug};
use crate::phy::bits::set8;
use crate::regs::Mmio;

/// The indirect LTE-coex registers: control (both grants), and the Wi-Fi and
/// Bluetooth TRX masks.
const LTE_COEX_CTRL: u16 = 0x38;
const LTE_WL_TRX_CTRL: u16 = 0xA0;
const LTE_BT_TRX_CTRL: u16 = 0xA4;
const GNT_SW_LOW: u32 = 0x1;
const GNT_SW_HIGH: u32 = 0x3;
/// `REG_WIFI_BT_INFO`: 0x2 | COEX_SCBD_ACTIVE | COEX_SCBD_ONOFF, with
/// BIT_BT_INT_EN.
const SCOREBOARD: u16 = 0x8000 | 0x0003;

/// Hand the antennas to Wi-Fi. False when the indirect window never
/// answered, so the caller can say so; the rest are plain register writes.
pub fn take_antenna<M: Mmio>(mmio: &M) -> bool {
    gnt_debug(mmio);
    let mut ok = write_field(mmio, LTE_COEX_CTRL, 1 << 7, 0); // BIT_LTE_COEX_EN
    ok &= write_field(mmio, LTE_WL_TRX_CTRL, 0xFFFF, 0xFFFF);
    ok &= write_field(mmio, LTE_BT_TRX_CTRL, 0xFFFF, 0xFFFF);
    cfg_init(mmio);
    // COEX_WLPRI_TX_RSP, _TX_BEACON and _TX_BEACONQ (REG_BT_COEX_TABLE_H).
    set8(mmio, 0x06CC, 1 << 3);
    set8(mmio, 0x06CC, 1 << 4);
    set8(mmio, 0x06CF, 1 << 3);
    ok &= write_field(mmio, LTE_COEX_CTRL, 0xC000, GNT_SW_LOW);
    ok &= write_field(mmio, LTE_COEX_CTRL, 0x0C00, GNT_SW_LOW);
    ok &= write_field(mmio, LTE_COEX_CTRL, 0x3000, GNT_SW_HIGH);
    ok &= write_field(mmio, LTE_COEX_CTRL, 0x0300, GNT_SW_HIGH);
    // REG_SYS_SDIO_CTRL + 3 |= BIT_LTE_MUX_CTRL_PATH >> 24: Wi-Fi owns the path.
    set8(mmio, 0x0073, 1 << 2);
    mmio.write16(0x00AA, SCOREBOARD);
    ok
}
