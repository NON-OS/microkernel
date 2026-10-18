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

//! An event's pointer at its edges, through every wait it can reach, a
//! command's among them: null, all ones, and not on a TRB boundary.

use super::boundary_through::{through_transfers, Seen, IGNORED};
use super::events::{command, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, SLOT, TRB};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::error::{XhciError, XhciResult};
use crate::trb::commands::noop_command;
use crate::trb::Trb;

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
