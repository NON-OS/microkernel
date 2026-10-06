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

//! CPlusCmd per version against Linux rtl_init_one (CPCMD_MASK) and
//! rtl_hw_start_8169.

use crate::chip::MacVersion;
use crate::hw::cplus_cmd;

#[test]
fn cplus_keeps_normal_mode_and_the_timer_and_drops_the_offloads() {
    // Normal_mode 1<<13, RxVlan 1<<6, RxChkSum 1<<5, INTT 0b11, PCIDAC 1<<4.
    assert_eq!(cplus_cmd(0x2073, MacVersion(46)), 0x2003);
    assert_eq!(cplus_cmd(0x2073, MacVersion(63)), 0x2003);
    // PCIMulRW 1<<3 on every 8169, EnAnaPLL 1<<14 on VER_02 and VER_03.
    assert_eq!(cplus_cmd(0, MacVersion(2)), 0x4008);
    assert_eq!(cplus_cmd(0, MacVersion(3)), 0x4008);
    assert_eq!(cplus_cmd(0, MacVersion(4)), 0x0008);
}
