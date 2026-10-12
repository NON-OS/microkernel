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

//! What ends the wait for Encryption Change (Vol 4, Part E, 7.7.8) other
//! than the change itself: the link dropping, or the poll budget.

use crate::encrypt_tests::LTK;
use crate::sim::link_events::{disconnected, encrypt_answered};
use crate::sim::stack::{stack, stack_with_budget};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn a_link_that_drops_ends_the_wait_and_its_event_stays_queued() {
    let start = op::LE_ENABLE_ENCRYPTION;
    // Connection Terminated due to MIC Failure (Vol 1, Part F, 2).
    let gone = disconnected(0x40, 0x3d);
    let mut s = stack(encrypt_answered(vec![disconnected(0x41, 0x13), gone.clone()]));
    assert_eq!(s.encrypt(0x40, &LTK), Err(HciError::Status(start, 0x3d)));
    assert_eq!(s.next_event(), Ok(Some(disconnected(0x41, 0x13))));
    assert_eq!(s.next_event(), Ok(Some(gone)));
}

#[test]
fn no_change_within_the_budget_is_a_timeout() {
    let mut s = stack_with_budget(encrypt_answered(vec![]), 5);
    let start = op::LE_ENABLE_ENCRYPTION;
    assert_eq!(s.encrypt(0x40, &LTK), Err(HciError::Timeout(start)));
}
