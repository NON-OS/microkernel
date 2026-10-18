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

//! A Command Ring Stopped event (completion code 24) is the ring's state,
//! not a command's completion: its pointer names the dequeue position, the
//! TRB the next command goes to (xHCI 1.2 section 4.6.1.2), so it must not
//! be taken as that command's answer.

use super::events::{command, CC_SUCCESS};
use super::fixture::{Fixture, SLOT};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::trb::commands::noop_command;

const RING_STOPPED: u8 = 24;
const ABORTED: u8 = 25;

#[test]
fn a_ring_stopped_event_naming_the_issued_trb_completes_nothing() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(command(issued, RING_STOPPED, 0));
    fx.hc.post(command(issued, CC_SUCCESS, SLOT));
    let got = wait_command_completion(fx.intr(), issued, &mut fx.ring);
    assert_eq!(got.map(|c| c.slot_id), Ok(SLOT));
}

#[test]
fn a_command_aborted_event_for_an_earlier_trb_completes_nothing() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let earlier = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(command(earlier, ABORTED, SLOT));
    fx.hc.post(command(issued, RING_STOPPED, 0));
    fx.hc.post(command(issued, CC_SUCCESS, SLOT));
    let got = wait_command_completion(fx.intr(), issued, &mut fx.ring);
    assert_eq!(got.map(|c| c.slot_id), Ok(SLOT));
}
