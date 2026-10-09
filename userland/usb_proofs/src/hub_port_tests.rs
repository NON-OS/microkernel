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

//! Hub port status words as USB 2.0 table 11-21 and USB 3.2 table 10-10
//! lay them out.

use crate::hub::port_status::{decode_port_status, SPEED_FULL, SPEED_HIGH, SPEED_LOW, SPEED_SUPER};

#[test]
fn usb2_port_status_gives_connection_enable_and_speed() {
    // Connected, enabled, powered, high speed; reset change set.
    let p = decode_port_status([0x03, 0x05, 0x10, 0x00], false);
    assert!(p.connected && p.enabled && p.powered && !p.resetting && p.reset_changed);
    assert_eq!(p.speed, SPEED_HIGH);
    assert_eq!(decode_port_status([0x03, 0x03, 0, 0], false).speed, SPEED_LOW);
    assert_eq!(decode_port_status([0x03, 0x01, 0, 0], false).speed, SPEED_FULL);
    let p = decode_port_status([0x11, 0x01, 0x01, 0x00], false);
    assert!(p.connected && p.resetting && !p.enabled && p.connect_changed);
}

#[test]
fn usb3_port_status_reads_power_from_bit_9_and_is_always_superspeed() {
    // Connected, enabled, link U0, powered.
    let p = decode_port_status([0x03, 0x02, 0x10, 0x00], true);
    assert!(p.connected && p.enabled && p.powered && p.reset_changed);
    assert_eq!(p.speed, SPEED_SUPER);
    assert!(!decode_port_status([0x01, 0x01, 0, 0], true).powered);
}
