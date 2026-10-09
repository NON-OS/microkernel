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

//! PHYstatus decoded against Linux's rtl8169_PHYstatus bits (FullDup 0x01,
//! LinkStatus 0x02, _10bps 0x04, _100bps 0x08, _1000bpsF 0x10) and the
//! 8125's _2500bpsF 0x400 from Realtek's r8125, and one line per change.

use nonos_libc::logged;

use super::memory::Memory;
use super::model::window;
use super::regmap_tests::RTL8125B;
use crate::chip::MacVersion;
use crate::constants::regs::REG_PHY_STATUS;
use crate::link::{decode, poll, LinkState};

fn up(speed: u32, full: bool) -> LinkState {
    LinkState { up: true, speed, full }
}

#[test]
fn each_speed_bit_decodes_including_2500_on_the_8125() {
    let (g, b) = (MacVersion(46), MacVersion(63));
    assert_eq!(decode(0x13, g), up(1000, true));
    assert_eq!(decode(0x0B, g), up(100, true));
    assert_eq!(decode(0x0A, g), up(100, false));
    assert_eq!(decode(0x06, g), up(10, false));
    assert_eq!(decode(0x03, g), up(0, true), "link without a speed bit");
    assert_eq!(decode(0x11, g), LinkState { up: false, speed: 0, full: false });
    assert_eq!(decode(0x0403, b), up(2500, true));
    assert_eq!(decode(0x0413, b), up(2500, true), "2500 wins over the 1000 bit");
    assert_eq!(decode(0x0403, g), up(0, true), "bit 10 is not PHYstatus on an 8168");
}

#[test]
fn a_change_is_logged_once_and_a_steady_link_is_not_logged_again() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver_with(&bar, RTL8125B);
    let before = logged().len();
    for raw in [0x0000u16, 0x0000, 0x0403, 0x0403, 0x0403, 0x0013, 0x0000] {
        bar.present16(REG_PHY_STATUS, raw);
        poll(&mut d);
    }
    let lines: Vec<String> = logged()[before..].to_vec();
    let want = [
        "rtl8169: link down\n",
        "rtl8169: link up 2500 full\n",
        "rtl8169: link up 1000 full\n",
        "rtl8169: link down\n",
    ];
    assert_eq!(lines, want);
}
