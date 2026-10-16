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

//! A command given up before the controller took it up. CRCR.CA then posts
//! no Command Aborted, only Command Ring Stopped, whose pointer is the
//! current dequeue: the given-up TRB itself, which the next doorbell would
//! run (xHCI 1.2 section 4.6.1.2). It is made a No Op with the ring's
//! cycle bit, as Linux's `trb_to_noop` does, so the controller runs nothing
//! the driver gave up on, here a Disable Slot it would have carried out
//! behind the slot table's back.

use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};

use super::command_at::{command_at, ring_base};
use super::fixture::Fixture;
use crate::constants::{CRCR_LO, TRB_TYPE_NOOP_CMD};
use crate::controller::command::run_command::run_command;
use crate::controller::program_command_ring;
use crate::error::XhciError;
use crate::trb::commands::disable_slot_command;

const CRCR_CA: u32 = 1 << 2;
const CRCR_CRR: u32 = 1 << 3;

#[test]
fn a_command_given_up_before_the_controller_took_it_is_not_left_to_run() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let (issued, cycle) = (ring_base(&cmd), cmd.cycle() != 0);
    let op = Arc::new(FakeBar::new(0x40));
    program_command_ring(op.base(), &mut cmd);
    op.present32(CRCR_LO as usize, CRCR_CRR);
    let _hc = run(&op, |bar: &FakeBar| {
        if bar.wrote32(CRCR_LO as usize) & CRCR_CA != 0 {
            bar.present32(CRCR_LO as usize, 0);
        }
    });
    let trb = disable_slot_command(cycle, 3);
    let got = run_command(fx.db(), fx.intr(), &mut cmd, &mut fx.ring, trb);
    assert_eq!(got.map(|_| ()), Err(XhciError::CommandCompletionTimeout));
    let left = command_at(issued);
    assert_eq!(left.get_type(), TRB_TYPE_NOOP_CMD, "the given-up Disable Slot was left to run");
    assert_eq!(left.get_cycle(), cycle, "a No Op the controller reads as written");
}
