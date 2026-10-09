// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! rtw88 `rtw_mac_pre_system_cfg` (mac.c:62) for the 8821C, a 3081-CPU chip
//! on PCIe. rtw88 runs it before every power-on (mac.c:382 and 390); the
//! register and bit values are from reg.h.

use crate::regs::Mmio;

const REG_SYS_FUNC_EN: usize = 0x0002;
const REG_RSV_CTRL: usize = 0x001C;
const REG_RF_CTRL: usize = 0x001F;
const REG_GPIO_MUXCFG: usize = 0x0040;
const REG_LED_CFG: usize = 0x004C;
const REG_PAD_CTRL1: usize = 0x0064;
const REG_HCI_OPT_CTRL: usize = 0x0074;
const REG_WLRF1: usize = 0x00EC;

/// Unlock the system registers, keep the PCIe link out of USB suspend, route
/// the PAPE/LNAON pins to the WLAN side, and hold baseband and RF in reset.
pub fn pre_system_cfg<M: Mmio>(mmio: &M) {
    mmio.write8(REG_RSV_CTRL, 0);
    // BIT_USB_SUS_DIS
    set32(mmio, REG_HCI_OPT_CTRL, 1 << 8, 0);
    // PIN mux: BIT_PAPE_WLBT_SEL | BIT_LNAON_WLBT_SEL, then clear
    // BIT_PAPE_SEL_EN | BIT_LNAON_SEL_EN, then BIT_WLRFE_4_5_EN.
    set32(mmio, REG_PAD_CTRL1, (1 << 29) | (1 << 28), 0);
    set32(mmio, REG_LED_CFG, 0, (1 << 25) | (1 << 26));
    set32(mmio, REG_GPIO_MUXCFG, 1 << 2, 0);
    // Disable BB/RF: BIT_FEN_BB_RSTB | BIT_FEN_BB_GLB_RST.
    let f = mmio.read8(REG_SYS_FUNC_EN);
    mmio.write8(REG_SYS_FUNC_EN, f & !0x03);
    // BIT_RF_SDM_RSTB | BIT_RF_RSTB | BIT_RF_EN.
    let r = mmio.read8(REG_RF_CTRL);
    mmio.write8(REG_RF_CTRL, r & !0x07);
    // BIT_WLRF1_BBRF_EN, bits 24..26.
    set32(mmio, REG_WLRF1, 0, 0x0700_0000);
}

fn set32<M: Mmio>(mmio: &M, off: usize, set: u32, clear: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, (v | set) & !clear);
}
