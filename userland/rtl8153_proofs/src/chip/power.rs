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

//! The model's state at power on. These are model values chosen so each
//! step of the bring-up has a bit to change and a test can see it: the
//! version in PLA_TCR1, the autoload done, out-of-band mode on with the
//! link list ready, the PHY on the LAN but in power down, ALDPS on, RX
//! aggregation's zero bit and VLAN stripping on, every frame accepted.
//! They are not a dump of a real chip.

use super::regs::{Regs, PHY, PLA, USB};

/// A locally administered test address, not a vendor's.
pub const TEST_MAC: [u8; 6] = [0x02, 0x00, 0x00, 0x81, 0x53, 0x01];

pub fn power_on(version: u16) -> Regs {
    let mut r = Regs::default();
    r.put(PLA, 0xe612, &version.to_le_bytes()); // PLA_TCR1
    r.put(PLA, 0xe004, &0x0002u16.to_le_bytes()); // AUTOLOAD_DONE
    r.put(PLA, 0xe84f, &[0x82]); // NOW_IS_OOB | LINK_LIST_READY
    r.put(PLA, 0xe8de, &0x4000u16.to_le_bytes()); // MCU_BORW_EN
    r.put(PLA, 0xe000, &0x0100u16.to_le_bytes()); // ALDPS left
    r.put(PLA, 0xc010, &0x0fu32.to_le_bytes()); // RCR_ACPT_ALL
    r.put(PLA, 0xe854, &0x0040u16.to_le_bytes()); // CPCR_RX_VLAN
    r.put(PLA, 0xd000, &TEST_MAC);
    r.put(USB, 0xd406, &0x0080u16.to_le_bytes()); // RX_ZERO_EN
    r.put(PHY, 0xa420, &0x0003u16.to_le_bytes()); // PHY_STAT_LAN_ON
    r.put(PHY, 0xa400, &0x1940u16.to_le_bytes()); // BMCR with PDOWN
    r.put(PHY, 0xa408, &0x0001u16.to_le_bytes()); // ANAR selector only
    r.put(PHY, 0xa412, &0x0100u16.to_le_bytes()); // 1000 half only
    r.put(PHY, 0xa430, &0x0004u16.to_le_bytes()); // EN_ALDPS
    r
}

/// The bits the chip clears by itself once it is done: CR_RST after the
/// MAC reset, BMCR_RESET after the PHY reset.
pub fn settle(r: &mut Regs) {
    let cr = r.byte(PLA, 0xe813);
    r.put(PLA, 0xe813, &[cr & !0x10]);
    let bmcr = r.word(PHY, 0xa400);
    r.put(PHY, 0xa400, &(bmcr & !0x8000).to_le_bytes());
}
