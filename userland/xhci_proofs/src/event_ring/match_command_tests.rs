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

//! Which event completes a command: the Command Completion Event whose
//! pointer names that command's TRB (`match_impostors` says how the others
//! are built).

use super::events::{command, port_change, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, EP0_DCI, SLOT};
use super::match_impostors::not_issued;
use crate::controller::wait_command_completion::wait_command_completion;
use crate::error::XhciError;
use crate::trb::commands::noop_command;

#[test]
fn a_completion_naming_another_command_does_not_complete_this_one() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let _earlier = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    for (n, p) in not_issued(issued).into_iter().enumerate() {
        fx.hc.post(command(p, CC_TRB_ERROR, 10 + n as u8));
    }
    fx.hc.post(transfer(issued, CC_STALL, 0, SLOT, EP0_DCI));
    fx.hc.post(port_change(1));
    fx.hc.post(command(issued, CC_SUCCESS, 3));
    let done = wait_command_completion(fx.intr(), issued, &mut fx.ring).expect("completes");
    assert_eq!(done.slot_id, 3, "the completion that names the command");
    assert_eq!(fx.hc.pending(), 0, "every event ahead of it was read and handed back");
}

#[test]
fn a_command_answered_only_for_others_times_out() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    for p in not_issued(issued) {
        fx.hc.post(command(p, CC_SUCCESS, 1));
    }
    let got = wait_command_completion(fx.intr(), issued, &mut fx.ring);
    assert_eq!(got.map(|c| c.slot_id), Err(XhciError::CommandCompletionTimeout));
}

#[test]
fn reserved_low_pointer_bits_do_not_hide_the_command_they_name() {
    // Bits 3:0 of the pointer are RsvdZ, which software ignores on read.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(command(issued | 0xF, CC_SUCCESS, 4));
    let done = wait_command_completion(fx.intr(), issued, &mut fx.ring).expect("completes");
    assert_eq!(done.slot_id, 4);
}
