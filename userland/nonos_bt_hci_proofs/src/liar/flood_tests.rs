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

//! Commands left unanswered and events in a flood. A wait ends after its
//! poll budget whatever comes meanwhile, keeping no more than `QUEUE_MAX`
//! events; a controller that never gives a command credit back is never
//! sent another command, and each one asked for ends in `Timeout`.

use crate::sim::config::Config;
use crate::sim::events::{adv_report, complete};
use crate::sim::stack::{opcodes, stack_with_budget};
use nonos_bt_hci::host::QUEUE_MAX;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn a_flood_while_a_command_waits_is_bounded_in_polls_and_in_memory() {
    let mut s = stack_with_budget(Config::default(), 200);
    s.bring_up().unwrap();
    s.transport().pipes().cfg.silent = true;
    for i in 0..1000 {
        s.transport().pipes().send_event(&adv_report(i as u8, &[]));
    }
    let left = s.transport().pipes().interrupt.len();
    assert_eq!(s.command(op::READ_BD_ADDR, &[]), Err(HciError::Timeout(op::READ_BD_ADDR)));
    assert!(s.transport().pipes().interrupt.len() < left, "the wait took the flood");
    assert!(!s.transport().pipes().interrupt.is_empty(), "and stopped within its budget");
    assert!(s.dropped() > 0);
    // What the wait kept, with the rest of the flood gone from the pipe.
    s.transport().pipes().interrupt.clear();
    let mut kept = 0;
    while s.next_event().unwrap().is_some() {
        kept += 1;
    }
    assert_eq!(kept, QUEUE_MAX);
}

#[test]
fn a_controller_that_gives_no_command_credit_back_is_sent_nothing_more() {
    let mut s = stack_with_budget(Config::default(), 200);
    s.bring_up().unwrap();
    let addr = complete(0, op::READ_BD_ADDR.0, 0, &[0; 6]);
    s.transport().pipes().cfg.script = vec![(op::READ_BD_ADDR.0, vec![addr])];
    s.command(op::READ_BD_ADDR, &[]).unwrap();
    let sent = opcodes(&mut s).len();
    for _ in 0..3 {
        let e = s.command(op::READ_LOCAL_VERSION, &[]);
        assert_eq!(e, Err(HciError::Timeout(op::READ_LOCAL_VERSION)));
    }
    assert_eq!(opcodes(&mut s).len(), sent, "nothing sent without a credit");
}
