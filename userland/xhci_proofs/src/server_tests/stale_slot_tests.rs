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

//! A slot id is its enabling client's, across a revival too. A revival
//! drops every slot, and the controller hands the same ids to whichever
//! client enables next: a class driver's stale cleanup, as usb_hid's
//! port-forget and usb_msc's probe do, then read or disabled another
//! driver's new disk or keyboard. Every request naming a slot is refused
//! unless the slot is the sender's own.

use super::bench::Bench;
use super::hc_device::lock;
use super::hc_reset::Resets;
use crate::conformance::model::CAP_LEN;
use crate::constants::{USBSTS, USBSTS_HSE};
use crate::protocol::{E_INVAL, OP_CONTROL_TRANSFER, OP_DISABLE_SLOT, OP_GET_DEVICE_DESCRIPTOR};
use crate::server::reply::set_sender;

pub const HIGH_SPEED: u32 = 3;
const STS: usize = CAP_LEN + USBSTS as usize;
pub const A: u32 = 0x5101;
pub const B: u32 = 0x5102;
pub const C: u32 = 0x5103;

/// Give the bench's controller a Host System Error and wait to see it.
pub fn fail_and_wait(b: &Bench) {
    lock(&b.hc).fault = true;
    while b.bar.wrote32(STS) & USBSTS_HSE == 0 {
        std::thread::yield_now();
    }
}

#[test]
fn a_slot_id_held_across_a_revival_reaches_no_other_clients_device() {
    let mut b = Bench::new(HIGH_SPEED);
    lock(&b.hc).resets = Resets::Followed;
    set_sender(A);
    let stale = b.root_device();
    fail_and_wait(&b);
    set_sender(B);
    let fresh = b.root_device();
    assert_eq!((fresh, b.mux.ctxs[0].driver.generation), (stale, 1), "the id handed out again");
    let commands = b.ran().len();
    set_sender(A);
    let get_status = [stale, 0, 0x80, 0, 0, 0, 0, 0, 2, 0];
    for (op, body) in [
        (OP_GET_DEVICE_DESCRIPTOR, &[stale][..]),
        (OP_CONTROL_TRANSFER, &get_status[..]),
        (OP_DISABLE_SLOT, &[stale][..]),
    ] {
        assert_eq!(b.call(op, body).status, E_INVAL, "op {op:#x} on another client's slot");
    }
    assert_eq!(b.ran().len(), commands, "nothing ran for A's stale id");
    set_sender(B);
    assert!(lock(&b.hc).enabled[fresh as usize], "B's slot is still enabled");
    assert_eq!(b.call(OP_GET_DEVICE_DESCRIPTOR, &[fresh]).status, 0, "and still B's");
    set_sender(0);
}
