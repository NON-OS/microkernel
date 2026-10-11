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

//! cfg2, read only under a version tg3 reads it for, and the LED modes it
//! carries as tg3_get_eeprom_hw_cfg maps them for a 57765-class part.

use crate::chip::hw_cfg::decode::decode;
use crate::constants::mi::*;
use crate::constants::sram::*;

const SIG: u32 = NIC_SRAM_DATA_SIG_MAGIC;
const VER: u32 = 1 << NIC_SRAM_DATA_VER_SHIFT;

#[test]
fn cfg2_only_under_a_version_tg3_reads() {
    let cfg2 = NIC_SRAM_DATA_CFG_2_APD_EN | NIC_SRAM_DATA_CFG_LED_MODE_PHY_2;
    // Unread, cfg2 is tg3's zero: no APD, and LED mode bits 0, the MAC's.
    for ver in [0, 0x100 << NIC_SRAM_DATA_VER_SHIFT, 0xFFFF_0000] {
        let hw = decode([SIG, 0, ver, cfg2, 0]).unwrap();
        assert!(!hw.apd, "{ver:#x}");
        assert_eq!(hw.led_ctrl, LED_CTRL_MODE_MAC);
    }
    let hw = decode([SIG, 0, 0xFF << NIC_SRAM_DATA_VER_SHIFT, cfg2, 0]).unwrap();
    assert!(hw.apd && hw.led_ctrl == LED_CTRL_MODE_PHY_2);
}

#[test]
fn led_modes_as_tg3_maps_them() {
    let both = LED_CTRL_MODE_PHY_1 | LED_CTRL_MODE_PHY_2;
    let cases = [
        (NIC_SRAM_DATA_CFG_LED_MODE_MAC, LED_CTRL_MODE_MAC),
        (0x4, LED_CTRL_MODE_PHY_1),
        (NIC_SRAM_DATA_CFG_LED_MODE_PHY_2, LED_CTRL_MODE_PHY_2),
        (SHASTA_EXT_LED_SHARED, LED_CTRL_MODE_SHARED | both),
        (SHASTA_EXT_LED_MAC, LED_CTRL_MODE_SHASTA_MAC),
        (SHASTA_EXT_LED_COMBO, LED_CTRL_MODE_COMBO | both),
        (0xC, LED_CTRL_MODE_PHY_1),
    ];
    for (cfg2, led) in cases {
        assert_eq!(decode([SIG, 0, VER, cfg2, 0]).unwrap().led_ctrl, led, "{cfg2:#x}");
    }
}
