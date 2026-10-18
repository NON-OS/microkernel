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

//! A full speed device below a hub is addressed with an 8-byte EP0, as one
//! on a root port is, and settles its size from its device descriptor.

use super::bench::Bench;
use super::hc_device::lock;
use super::hc_setup::Setup;
use super::hub_bench::{FULL, HIGH};

const EVALUATE_CONTEXT: u32 = 13;

#[test]
fn a_full_speed_device_below_a_hub_gets_its_ep0_size_from_its_descriptor() {
    let mut b = Bench::new(HIGH as u32);
    let hub = b.root_device();
    assert_eq!(b.configure_hub(hub, 4, 0, 0), 0);
    lock(&b.hc).answer = |s: &Setup| {
        let mut d = vec![18, 1, 0, 2, 0, 0, 0, 64];
        d.resize(s.length as usize, 0);
        d
    };
    let kbd = b.enable_slot();
    let r = b.routed(kbd, FULL, 0x1, (hub, 1));
    assert_eq!(r.status, 0);
    assert_eq!(u16::from_le_bytes([r.body[4], r.body[5]]), 64, "bMaxPacketSize0");
    assert_eq!(b.last(EVALUATE_CONTEXT).trb.slot_id(), kbd);
}
