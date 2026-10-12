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

//! Command flow control (Vol 4, Part E, 4.4) and the events that come
//! while a command is waited for.

use crate::sim::config::{Config, ADDR};
use crate::sim::events::{adv_report, complete, status};
use crate::sim::stack::stack;
use nonos_bt_hci::host::QUEUE_MAX;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn no_command_is_sent_while_the_controller_takes_none() {
    let mut s = stack(Config { silent: true, ..Config::default() });
    let p = s.transport().pipes();
    p.send_event(&complete(0, op::RESET.0, 0, &[]));
    p.send_event(&complete(1, 0x0000, 0, &[]));
    p.send_event(&complete(1, op::READ_BD_ADDR.0, 0, &ADDR));
    s.command(op::RESET, &[]).unwrap();
    assert_eq!(s.command(op::READ_BD_ADDR, &[]).unwrap(), ADDR);
    // Sent after the opcode-0 event was read: only the answer still waited.
    assert_eq!(s.transport().pipes().commands[1].2, 1);
}

#[test]
fn command_status_takes_a_pending_command_and_refuses_a_completing_one() {
    let mut s = stack(Config { silent: true, ..Config::default() });
    let create = op::LE_CREATE_CONNECTION;
    s.transport().pipes().send_event(&status(0, 1, create.0));
    assert_eq!(s.command_pending(create, &[]), Ok(()));
    s.transport().pipes().send_event(&status(0x0c, 1, create.0));
    assert_eq!(s.command_pending(create, &[]), Err(HciError::Status(create, 0x0c)));
    s.transport().pipes().send_event(&status(0, 1, op::RESET.0));
    assert_eq!(s.command(op::RESET, &[]), Err(HciError::Status(op::RESET, 0)));
}

#[test]
fn events_met_while_waiting_are_kept_in_order() {
    let before = vec![adv_report(1, &[2, 1, 6]), adv_report(2, &[])];
    let mut s = stack(Config { before: before.clone(), ..Config::default() });
    s.bring_up().unwrap();
    for _ in 0..7 {
        assert_eq!(s.next_event().unwrap().as_ref(), Some(&before[0]));
        assert_eq!(s.next_event().unwrap().as_ref(), Some(&before[1]));
    }
    assert_eq!(s.next_event(), Ok(None));
    assert_eq!(s.dropped(), 0);
}

#[test]
fn a_flood_of_reports_keeps_only_the_newest() {
    let before: Vec<Vec<u8>> = (0..70u8).map(|i| adv_report(i, &[])).collect();
    let mut s = stack(Config { before: before.clone(), ..Config::default() });
    s.command(op::RESET, &[]).unwrap();
    assert_eq!(s.dropped(), 70 - QUEUE_MAX as u32);
    for want in &before[70 - QUEUE_MAX..] {
        assert_eq!(s.next_event().unwrap().as_ref(), Some(want));
    }
    assert_eq!(s.next_event(), Ok(None));
}
