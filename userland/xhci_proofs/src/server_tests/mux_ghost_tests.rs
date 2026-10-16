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

//! A controller gone from the bus reads all ones, every PORTSC showing a
//! device connected and enabled at speed 15. The multiplexer reads the
//! ports of every controller, not only of one a request was sent to, so a
//! second controller that failed before anything was sent to it must show
//! nothing attached too: class drivers would enumerate ghosts.

use super::bench::Bench;
use crate::conformance::model::{CAP_LEN, PORT1};
use crate::constants::{PORT_REG_STRIDE, USBSTS};
use crate::protocol::OP_PORT_STATUS;

const HIGH_SPEED: u32 = 3;

#[test]
fn a_controller_gone_from_the_bus_shows_no_ghost_ports() {
    let mut b = Bench::new(HIGH_SPEED);
    let (second, _intr) = b.second_controller();
    second.present32(CAP_LEN + USBSTS as usize, u32::MAX);
    for port in 0..2 {
        second.present32(PORT1 + port * PORT_REG_STRIDE as usize, u32::MAX);
    }
    let ports = b.call(OP_PORT_STATUS, &[]);
    assert_eq!((ports.status, ports.body[0]), (0, 4), "both controllers' ports");
    let entries: Vec<_> = ports.body[4..].chunks(8).collect();
    assert_ne!(entries[0][4..8], [0; 4], "the first controller's device is still seen");
    for entry in &entries[2..] {
        assert_eq!(entry[4..8], [0; 4], "port {}: nothing attached", entry[0]);
    }
}
