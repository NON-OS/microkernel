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

//! A device taken out while nothing plays is noticed (the lead's review,
//! R19): the bound port's Current Connect Status read from driver.xhci0's
//! OP_PORT_STATUS data. Connect Status Change alone does not count, and a
//! port the data does not name has nothing on it.

use crate::portsc::{connected_in, speed_in};

fn data(entries: &[(u8, u32)]) -> Vec<u8> {
    let mut d = vec![entries.len() as u8, 0, 0, 0];
    for &(port, sc) in entries {
        d.extend_from_slice(&[port, 0, 0, 0]);
        d.extend_from_slice(&sc.to_le_bytes());
    }
    d
}

#[test]
fn the_bound_port_is_read_by_its_connect_status() {
    let d = data(&[(1, 0x0000_0000), (3, 0x0000_0C01), (5, 1 << 17)]);
    assert!(connected_in(&d, 3));
    assert_eq!(speed_in(&d, 3), Some(3), "the speed read is unchanged");
    assert!(!connected_in(&d, 1), "nothing on it");
    assert!(!connected_in(&d, 5), "a change without a device is not a device");
    assert!(!connected_in(&d, 7), "a port not listed");
}

#[test]
fn short_data_names_no_device() {
    let d = data(&[(3, 1)]);
    assert!(!connected_in(&d[..8], 3));
    assert!(!connected_in(&[], 3));
}
