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

//! While the controller refuses every recovery, each STALLed request's
//! buffer is kept with EP0's ring, and the ring refuses requests once it is
//! full: no more are kept than half the TRBs it holds for the controller.
//! A request the full ring refuses leaves none of its TRBs behind: queued
//! in part, its data stage named the buffer freed with the refusal.

use nonos_libc::dma_host;

use super::bench::Bench;
use super::hc_device::lock;
use super::kept_bound_tests::{ep0, kept, GET_STATUS, HIGH_SPEED};
use crate::constants::{TRANSFER_RING_TRBS, TRB_BYTES, TRB_TYPE_DATA_STAGE};
use crate::protocol::{OP_CONTROL_TRANSFER, OP_GET_DEVICE_DESCRIPTOR};
use crate::trb::read_volatile_at;

/// What a ring holds for the controller at once: all but the Link and one.
const MAX_PENDING: usize = TRANSFER_RING_TRBS - 2;

#[test]
fn buffers_kept_while_recovery_keeps_failing_stay_bounded_and_named() {
    for op in [OP_CONTROL_TRANSFER, OP_GET_DEVICE_DESCRIPTOR] {
        bounded_and_named(op);
    }
}

/// 100 STALLed requests `op` on EP0, its recovery refused every time.
fn bounded_and_named(op: u16) {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.root_device();
    (lock(&b.hc).stall_in, lock(&b.hc).refuse_recovery) = (true, true);
    let from = ep0(&mut b, slot).enqueue_phys();
    assert_eq!(from, ep0(&mut b, slot).phys(), "EP0 starts at its first TRB: no lap below");
    let body =
        if op == OP_CONTROL_TRANSFER { [&[slot][..], &GET_STATUS].concat() } else { vec![slot] };
    for round in 0..100 {
        let _ = b.call(op, &body);
        let n = kept(ep0(&mut b, slot));
        assert!(n <= MAX_PENDING / 2, "op {op:#x} round {round}: {n} kept");
    }
    let ring = ep0(&mut b, slot);
    for at in (from..ring.enqueue_phys()).step_by(TRB_BYTES) {
        // SAFETY: `dma_host` names the ring's live grant, a whole TRB at `at`.
        let trb = unsafe { read_volatile_at(dma_host(at).expect("EP0's ring")) };
        let named = trb.get_pointer();
        let data = trb.get_type() == TRB_TYPE_DATA_STAGE;
        assert!(!data || dma_host(named).is_some(), "op {op:#x}: a TRB names freed {named:#x}");
    }
}
