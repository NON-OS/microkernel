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

//! The named edges of an event TRB, each put through every wait it can
//! reach: slot 0 and 255, endpoint 0 and 31, a residual of u32::MAX read as
//! 24 bits, a null pointer, a pointer of all ones, and pointers that are not
//! on a TRB boundary.

use super::events::{command, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, BULK_IN_DCI, EP0_DCI, INT_DCI, SLOT, TRB};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::controller::{
    bulk_transfer, get_device_descriptor, poll_interrupt_in, IntrPoll, DEVICE_DESCRIPTOR_LEN,
};
use crate::error::{XhciError, XhciResult};
use crate::trb::commands::noop_command;
use crate::trb::Trb;

/// What each transfer consumer made of one edge event aimed at its own TRB:
/// the interrupt poll's single answer for an 8-byte request (`None` while
/// pending), then, each with the genuine event posted behind the edge, a
/// 512-byte bulk transfer whose genuine event leaves 100 bytes behind and a
/// device descriptor read whose genuine event succeeds.
#[derive(Debug, PartialEq)]
struct Seen {
    /// The report handed up, none, or the error the report ended in.
    poll: XhciResult<Option<u16>>,
    bulk: XhciResult<u32>,
    control: XhciResult<usize>,
}

/// The edge was not taken by any of them.
const IGNORED: Seen =
    Seen { poll: Ok(None), bulk: Ok(412), control: Ok(DEVICE_DESCRIPTOR_LEN as usize) };

fn through_transfers(edge: impl Fn(u64, u8) -> Trb) -> Seen {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(edge(issued, INT_DCI));
    let poll = match poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8) {
        Ok(IntrPoll::Complete(n)) => Ok(Some(n)),
        Ok(IntrPoll::Pending) => Ok(None),
        Err(e) => Err(e),
    };
    fx.dequeue_bus();

    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    fx.hc.post(edge(issued, BULK_IN_DCI));
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let bulk = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    fx.dequeue_bus();

    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let status = ep0.enqueue_phys() + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    fx.hc.post(edge(status, EP0_DCI));
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let control = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    fx.dequeue_bus();

    Seen { poll, bulk, control }
}

/// The slot id a command wait returns with `edge` ahead of the genuine
/// completion, which names slot 3.
fn through_command(edge: impl Fn(u64) -> Trb) -> XhciResult<u8> {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(edge(issued));
    fx.hc.post(command(issued, CC_SUCCESS, 3));
    wait_command_completion(fx.intr(), issued, &mut fx.ring).map(|c| c.slot_id)
}

#[test]
fn slot_0_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, ep| transfer(p, CC_STALL, 1, 0, ep)), IGNORED);
}

#[test]
fn slot_255_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, ep| transfer(p, CC_STALL, 1, 255, ep)), IGNORED);
}

#[test]
fn endpoint_0_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, _| transfer(p, CC_STALL, 1, SLOT, 0)), IGNORED);
}

#[test]
fn endpoint_31_is_taken_by_no_transfer_wait() {
    assert_eq!(through_transfers(|p, _| transfer(p, CC_STALL, 1, SLOT, 31)), IGNORED);
}

#[test]
fn a_residual_of_u32_max_is_read_as_24_bits_and_clamped_to_the_request() {
    let widest = through_transfers(|p, ep| transfer(p, CC_SUCCESS, u32::MAX, SLOT, ep));
    let taken = Seen { poll: Ok(Some(0)), bulk: Ok(0), control: Ok(DEVICE_DESCRIPTOR_LEN as usize) };
    assert_eq!(widest, taken);

    let all_ones = through_transfers(|p, ep| {
        let mut t = transfer(p, CC_SUCCESS, 0, SLOT, ep);
        t.d2 = u32::MAX;
        t
    });
    // Completion code 0xFF is no success: the report is that error, not a
    // report of no bytes, so the server recovers the endpoint.
    let failed = XhciError::TransferCompletionFailed(0xFF);
    assert_eq!(all_ones, Seen { poll: Err(failed), bulk: Err(failed), control: Err(failed) });
}

#[test]
fn a_null_pointer_completes_nothing() {
    assert_eq!(through_transfers(|_, ep| transfer(0, CC_STALL, 1, SLOT, ep)), IGNORED);
    assert_eq!(through_command(|_| command(0, CC_TRB_ERROR, 9)), Ok(3));
}

#[test]
fn a_pointer_of_all_ones_completes_nothing() {
    assert_eq!(through_transfers(|_, ep| transfer(u64::MAX, CC_STALL, 1, SLOT, ep)), IGNORED);
    assert_eq!(through_command(|_| command(u64::MAX, CC_TRB_ERROR, 9)), Ok(3));
}

#[test]
fn a_misaligned_pointer_names_the_trb_it_falls_in_and_no_other() {
    // Bits 3:0 are reserved: inside the issued TRB the event is its own.
    let inside = through_transfers(|p, ep| transfer(p | 0x8, CC_STALL, 1, SLOT, ep));
    let stalled = XhciError::TransferCompletionFailed(CC_STALL);
    assert_eq!(inside, Seen { poll: Err(stalled), bulk: Err(stalled), control: Err(stalled) });
    assert_eq!(
        through_command(|p| command(p | 0x7, CC_TRB_ERROR, 9)),
        Err(XhciError::CommandCompletionFailed(CC_TRB_ERROR))
    );

    // Eight bytes either side lands in a neighbour, which is not the TRB.
    let after = through_transfers(|p, ep| transfer(p.wrapping_add(TRB + 8), CC_STALL, 1, SLOT, ep));
    assert_eq!(after, IGNORED, "the TRB after");
    // The TRB before a control transfer's status stage is its data stage: a
    // STALL there is this transfer's, and ends it. Neither of the others has
    // a TRB of its own there.
    let before = through_transfers(|p, ep| transfer(p.wrapping_sub(8), CC_STALL, 1, SLOT, ep));
    assert_eq!(before, Seen { control: Err(stalled), ..IGNORED }, "the TRB before");
    for shift in [TRB + 8, 8u64.wrapping_neg()] {
        assert_eq!(through_command(|p| command(p.wrapping_add(shift), CC_TRB_ERROR, 9)), Ok(3));
    }
}
