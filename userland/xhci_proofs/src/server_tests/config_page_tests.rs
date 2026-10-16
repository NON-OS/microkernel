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

//! A configuration descriptor is read for any index the client names (the
//! device answers one it lacks with a STALL) and up to a page: composite
//! devices, webcams and docks among them, often pass the 512 bytes and the
//! index 0 the reply was held to.

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{OP_CONTROL_TRANSFER, OP_GET_CONFIG_DESCRIPTOR};
use crate::server::runner::TX_LEN;

const HIGH_SPEED: u32 = 3;

#[test]
fn any_configuration_up_to_a_page_is_read() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    // The device answers with the setup packet's wValue, then fills to 3000.
    lock(&b.hc).answer = |s| {
        let mut d = s.value.to_le_bytes().to_vec();
        d.resize(s.length.min(3000) as usize, 0x55);
        d
    };
    let r = b.call(OP_GET_CONFIG_DESCRIPTOR, &[slot, 1, 0x00, 0x10]);
    assert_eq!(r.status, 0, "index 1");
    assert_eq!(u16::from_le_bytes([r.body[0], r.body[1]]), 3000, "past the old 512 cap");
    assert_eq!(r.body[4..6], [1, 2], "GET_DESCRIPTOR(Configuration, 1)");
    let r = b.call(OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, 0xFF, 0xFF]);
    assert_eq!(u16::from_le_bytes([r.body[0], r.body[1]]), 3000, "65535 asked, a page read");
}

#[test]
fn a_device_that_sends_past_a_page_is_read_to_a_page_and_the_reply_fits_the_runner() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    lock(&b.hc).answer = |_| vec![0x55; 65535];
    let r = b.call(OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, 0xFF, 0xFF]);
    assert_eq!((r.status, u16::from_le_bytes([r.body[0], r.body[1]])), (0, 4096));
    assert_eq!(24 + r.body.len(), TX_LEN, "the runner's whole reply buffer, no more");
    let get = [slot, 0, 0x80, 6, 0, 2, 0, 0, 0xFF, 0xFF];
    let r = b.call(OP_CONTROL_TRANSFER, &get);
    assert_eq!((r.status, u16::from_le_bytes([r.body[0], r.body[1]])), (0, 4096));
    assert!(24 + r.body.len() <= TX_LEN);
}
