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

//! A bulk pipe that had not halted is dropped and added again, so the host
//! starts from toggle zero as the device does after CLEAR_FEATURE.

use super::command_at::{command_at, ring_base, CONFIGURE_ENDPOINT, SET_TR_DEQUEUE, STOP_ENDPOINT};
use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, BULK_OUT_DCI, SLOT, TRB};
use super::toggle_slot::{dw, storage_slot, CONTEXT_SIZE};
use crate::controller::command::recovered::CC_CONTEXT_STATE_ERROR;
use crate::controller::reset_bulk_endpoint;
use crate::trb::Trb;

#[test]
fn a_bulk_pipe_that_had_not_halted_is_dropped_and_added_again() {
    // The case that kept a USB stick out of step: the pipe timed out, was
    // only stopped, and CLEAR_FEATURE set the device's toggle back to DATA0.
    for (dir_in, dci) in [(true, BULK_IN_DCI), (false, BULK_OUT_DCI)] {
        let mut fx = Fixture::new();
        let mut cmd = fx.command_ring();
        let mut res = storage_slot(&fx);
        // A few transfers already went through, so the ring is mid-way.
        {
            let pipes = res.bulk.as_mut().expect("pipes");
            let ring = if dir_in { &mut pipes.in_ring } else { &mut pipes.out_ring };
            for _ in 0..5 {
                ring.enqueue(Trb::zero()).expect("room");
            }
        }
        let base = ring_base(&cmd);
        fx.hc.post(command(base, CC_CONTEXT_STATE_ERROR, SLOT));
        fx.hc.post(command(base + TRB, CC_SUCCESS, SLOT));
        fx.hc.post(command(base + 2 * TRB, CC_SUCCESS, SLOT));
        fx.hc.post(command(base + 3 * TRB, CC_SUCCESS, SLOT));
        reset_bulk_endpoint(
            fx.db(),
            fx.intr(),
            &mut cmd,
            &mut fx.ring,
            &res,
            CONTEXT_SIZE,
            (false, dir_in),
        )
        .expect("reset");
        assert_eq!(command_at(base + TRB).get_type(), STOP_ENDPOINT);
        assert_eq!(command_at(base + 2 * TRB).get_type(), SET_TR_DEQUEUE);
        let configure = command_at(base + 3 * TRB);
        assert_eq!(configure.get_type(), CONFIGURE_ENDPOINT);
        assert_eq!(configure.slot_id(), SLOT);
        assert_eq!(configure.get_pointer(), res.input_context.phys());

        let input = &res.input_context;
        assert_eq!(dw(input, 0, 0), 1 << dci, "dropped");
        assert_eq!(dw(input, 0, 1), 1 | 1 << dci, "slot and endpoint added");
        assert!(dw(input, 1, 0) >> 27 >= dci as u32, "Context Entries covers it");
        let ep = dci as usize + 1;
        assert_eq!(dw(input, ep, 0) & 7, 0, "the state is not handed back");
        let kept = 3 << 1 | 6 << 3 | 15 << 8 | 1024 << 16;
        assert_eq!(dw(input, ep, 1), kept, "type, burst, mps kept");
        let pipes = res.bulk.as_ref().expect("pipes");
        let ring = if dir_in { &pipes.in_ring } else { &pipes.out_ring };
        let deq = dw(input, ep, 2) as u64 | (dw(input, ep, 3) as u64) << 32;
        assert_eq!(deq, ring.enqueue_phys() | ring.cycle() as u64, "the ring goes on from here");
        assert_eq!(dw(input, ep, 4), 3 * 1024);
        let other = if dir_in { BULK_OUT_DCI } else { BULK_IN_DCI } as usize + 1;
        assert_eq!(dw(input, other, 1), 0, "the other pipe is left alone");
    }
}
