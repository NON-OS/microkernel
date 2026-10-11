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

//! The pure halves of the copper link held to Linux over every input:
//! tg3_aux_stat_to_speed_duplex, what tg3_phy_autoneg_cfg advertises,
//! tg3_phy_copper_an_config_ok, and mii_resolve_flowctrl_fdx's table.

use crate::constants::phy::*;
use crate::phy::pause::{flow_control, FLOW_CTRL_RX, FLOW_CTRL_TX};
use crate::phy::resolve::{advertise, advertising_ok, speed_duplex};

#[test]
fn every_aux_status_speed() {
    let want = [None, Some((10, false)), Some((10, true)), Some((100, false)), None];
    for (code, w) in want.iter().enumerate() {
        assert_eq!(speed_duplex((code as u32) << 8 | 0xF8FF), *w, "{code}");
    }
    assert_eq!(speed_duplex(0x0500), Some((100, true)));
    assert_eq!(speed_duplex(0x0600), Some((1000, false)));
    assert_eq!(speed_duplex(0x0700), Some((1000, true)));
}

#[test]
fn advertisement_and_its_check() {
    assert_eq!(advertise(false), (0x05E1, Some(0x0300)));
    assert_eq!(advertise(true), (0x05E1, None));
    assert!(advertising_ok(0x05E1, Some(0x0300), true, false));
    assert!(advertising_ok(0x01E1, Some(0x0300), false, false), "half ignores pause");
    assert!(!advertising_ok(0x01E1, Some(0x0300), true, false), "full needs pause");
    assert!(!advertising_ok(0x0DE1, Some(0x0300), true, false), "asym pause is not asked");
    assert!(!advertising_ok(0x05E1, Some(0x0200), true, false), "both gigabit modes");
    assert!(!advertising_ok(0x05A1, Some(0x0300), true, false), "every 10/100 mode");
    assert!(advertising_ok(0x05E1, None, true, true), "10/100 part");
}

#[test]
fn pause_resolution_is_the_mii_table() {
    let (cap, asym) = (ADVERTISE_PAUSE_CAP, ADVERTISE_PAUSE_ASYM);
    let both = FLOW_CTRL_TX | FLOW_CTRL_RX;
    let table = [
        (cap, cap, both),
        (cap, cap | asym, both),
        (cap | asym, cap, both),
        (cap | asym, asym, FLOW_CTRL_RX),
        (asym, cap | asym, FLOW_CTRL_TX),
        (asym, asym, 0),
        (cap, asym, 0),
        (asym, cap, 0),
        (0, cap | asym, 0),
    ];
    for (lcl, rmt, fc) in table {
        assert_eq!(flow_control(lcl, rmt), fc, "{lcl:#x} {rmt:#x}");
    }
}
