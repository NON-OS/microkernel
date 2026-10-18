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

//! A request buffer kept because its endpoint could not be brought back
//! stays only until the controller has passed its TRBs: a transfer that
//! completes on the ring, or a dequeue moved, frees every one kept before.
//! Kept until Disable Slot, they grew by one for every request whose
//! recovery the controller refused.

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::OP_CONTROL_TRANSFER;
use crate::rings::transfer::TransferRing;

pub const HIGH_SPEED: u32 = 3;
pub const GET_STATUS: [u8; 9] = [0, 0x80, 0, 0, 0, 0, 0, 2, 0];

/// How many buffers `ring` keeps.
pub fn kept(ring: &TransferRing) -> usize {
    let bufs = ring.kept.take();
    let n = bufs.len();
    ring.kept.set(bufs);
    n
}

pub fn ep0(b: &mut Bench, slot: u8) -> &TransferRing {
    let d = &mut b.mux.ctxs[0].driver;
    &d.slots.resources_mut(slot, d.layout.max_slots).expect("the slot").ep0
}

/// GET_STATUS on `slot`'s EP0, as a class driver sends it.
pub fn get_status(b: &mut Bench, slot: u8) -> i32 {
    let mut body = vec![slot];
    body.extend_from_slice(&GET_STATUS);
    b.call(OP_CONTROL_TRANSFER, &body).status
}

#[test]
fn a_buffer_kept_for_a_failed_recovery_goes_with_the_next_completed_transfer() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    lock(&b.hc).refuse_recovery = true;
    for round in 0..100 {
        lock(&b.hc).stall_in = true;
        let _ = get_status(&mut b, slot);
        assert_eq!(kept(ep0(&mut b, slot)), 1, "round {round}: kept while EP0 is halted");
        lock(&b.hc).stall_in = false;
        assert_eq!(get_status(&mut b, slot), 0, "round {round}");
        assert_eq!(kept(ep0(&mut b, slot)), 0, "round {round}: freed once the ring moved on");
    }
}
