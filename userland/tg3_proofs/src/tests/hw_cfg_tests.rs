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

//! The board facts read out of SRAM, held to tg3_get_eeprom_hw_cfg: a
//! missing signature is a LOM with the PHY_1 LEDs, a fiber port or the APE
//! is refused, and each flag follows its own bit.

use crate::chip::hw_cfg::cfg::NO_SIGNATURE;
use crate::chip::hw_cfg::decode::decode;
use crate::constants::mi::*;
use crate::constants::sram::*;

const SIG: u32 = NIC_SRAM_DATA_SIG_MAGIC;
/// Version 1 in the high half: cfg2 is read.
const VER: u32 = 1 << NIC_SRAM_DATA_VER_SHIFT;

#[test]
fn no_signature_is_linuxs_default() {
    for sig in [0, !SIG, SIG ^ 1, 0xFFFF_FFFF] {
        assert_eq!(decode([sig, !0, VER, !0, !0]), Ok(NO_SIGNATURE));
    }
    const { assert!(NO_SIGNATURE.eeprom_write_protect && !NO_SIGNATURE.asf) };
    assert_eq!(NO_SIGNATURE.led_ctrl, LED_CTRL_MODE_PHY_1);
}

#[test]
fn fiber_and_ape_are_refused() {
    let fiber = NIC_SRAM_DATA_CFG_PHY_TYPE_FIBER;
    assert!(decode([SIG, fiber, VER, 0, 0]).is_err());
    assert!(decode([SIG, NIC_SRAM_DATA_CFG_APE_ENABLE, VER, 0, 0]).is_err());
    // The other PHY types are copper.
    for t in [0x00, 0x10, 0x30] {
        assert!(decode([SIG, t, VER, 0, 0]).is_ok(), "{t:#x}");
    }
}

#[test]
fn flags_follow_their_bits() {
    let nic = NIC_SRAM_DATA_CFG_ASF_ENABLE | NIC_SRAM_DATA_CFG_EEPROM_WP;
    let cfg2 = NIC_SRAM_DATA_CFG_2_APD_EN | NIC_SRAM_DATA_CFG_2_CAPACITIVE;
    let hw = decode([SIG, nic, VER, cfg2, NIC_SRAM_LNK_FLAP_AVOID]).unwrap();
    assert!(hw.asf && hw.eeprom_write_protect && hw.apd);
    assert!(hw.capacitive_coupling && hw.keep_link);
    let hw = decode([SIG, 0, VER, 0, 0]).unwrap();
    assert!(!hw.asf && !hw.eeprom_write_protect && !hw.apd && !hw.keep_link);
}
