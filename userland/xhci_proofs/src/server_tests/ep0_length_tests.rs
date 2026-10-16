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

//! An EP0 reply carries the bytes the device sent, no more: the IN data
//! stage sets ISP and the Short Packet event's residual is taken off the
//! request (xHCI 1.2 section 4.10.1.1). The requested length used to be
//! reported, padded with zeros, so a class driver could not tell a 9-byte
//! answer from a 512-byte one. A host-to-device data stage, which these
//! requests carry no bytes for, is refused: it went out as zeros and was
//! reported a success.

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{
    E_INVAL, OP_CONTROL_TRANSFER, OP_GET_CONFIG_DESCRIPTOR, OP_GET_DEVICE_DESCRIPTOR,
};

const HIGH_SPEED: u32 = 3;

#[test]
fn a_short_answer_is_reported_as_the_bytes_the_device_sent() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    lock(&b.hc).answer = |s| (0..s.length.min(9)).map(|i| 0xA0 + i as u8).collect();
    let r = b.call(OP_CONTROL_TRANSFER, &[slot, 0, 0x80, 0, 0, 0, 0, 0, 64, 0]);
    assert_eq!((r.status, &r.body[..2]), (0, &[9u8, 0][..]), "nine bytes moved");
    assert_eq!(r.body[2..], [0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8]);
    let r = b.call(OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, 0, 2]);
    assert_eq!((r.status, r.body[0], r.body.len()), (0, 9, 4 + 9));
    let r = b.call(OP_GET_DEVICE_DESCRIPTOR, &[slot]);
    assert_eq!((r.status, r.body.len()), (0, 9), "a short device descriptor as it came");
}

#[test]
fn a_host_to_device_data_stage_is_refused_before_anything_is_queued() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    let ep0 = |b: &mut Bench| {
        b.mux.ctxs[0].driver.slots.resources_mut(slot, 8).map(|r| r.ep0.enqueue_phys())
    };
    let before = ep0(&mut b);
    let set_report = [slot, 0, 0x21, 0x09, 0, 2, 0, 0, 1, 0];
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &set_report).status, E_INVAL);
    assert_eq!(ep0(&mut b), before, "no TRB queued");
    let set_idle = [slot, 0, 0x21, 0x0A, 0, 0, 0, 0, 0, 0];
    assert_eq!(b.call(OP_CONTROL_TRANSFER, &set_idle).status, 0, "no data stage is fine");
}
