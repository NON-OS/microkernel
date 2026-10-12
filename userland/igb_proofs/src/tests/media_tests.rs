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
//! The port's media from CTRL_EXT's link mode: copper is driven, every other
//! mode refused by name.

use crate::bring::port::{copper, refusal, KX, SERDES, SGMII};
use crate::facts::mac::CTRL_EXT_LINK_MODE_MASK;
use crate::facts::regs::REG_CTRL_EXT;

use super::model::{regs, window};

#[test]
fn link_mode_vectors() {
    assert_eq!(CTRL_EXT_LINK_MODE_MASK, 0x00C0_0000, "E1000_CTRL_EXT_LINK_MODE_MASK");
    assert_eq!(refusal(0), None, "GMII: copper behind the internal PHY");
    assert_eq!(refusal(0x0040_0000), Some(KX), "E1000_CTRL_EXT_LINK_MODE_1000BASE_KX");
    assert_eq!(refusal(0x0080_0000), Some(SGMII), "E1000_CTRL_EXT_LINK_MODE_SGMII");
    assert_eq!(refusal(0x00C0_0000), Some(SERDES), "E1000_CTRL_EXT_LINK_MODE_PCIE_SERDES");
    assert_eq!(refusal(!CTRL_EXT_LINK_MODE_MASK), None, "only the two mode bits count");
}

#[test]
fn the_register_is_what_is_read() {
    let bar = window();
    bar.present32(REG_CTRL_EXT, 0x1000_0000);
    assert_eq!(copper(&regs(&bar)), Ok(()));
    bar.present32(REG_CTRL_EXT, 0x00C0_0000);
    assert_eq!(copper(&regs(&bar)), Err(SERDES));
    for e in [KX, SGMII, SERDES] {
        assert!(e.ends_with("not driven: copper only"), "{e}");
        assert!("igb: ".len() + e.len() < 128, "fits one console line");
    }
}
