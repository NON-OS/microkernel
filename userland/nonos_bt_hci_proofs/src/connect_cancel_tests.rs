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

//! LE Create Connection Cancel (Vol 4, Part E, 7.8.13): an attempt the
//! peer does not answer within the poll budget is cancelled, and a cancel
//! that comes after the link is up is disallowed and the link kept.

use crate::sim::config::Config;
use crate::sim::events::{complete, status};
use crate::sim::link_events::{connected, CONNECT};
use crate::sim::stack::stack_with_budget;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

const CREATE: u16 = op::LE_CREATE_CONNECTION.0;
const CANCEL: u16 = op::LE_CREATE_CONNECTION_CANCEL.0;

#[test]
fn a_peer_not_heard_is_cancelled_and_its_unknown_connection_taken() {
    let cancelled = vec![complete(1, CANCEL, 0, &[]), connected(0x02, 0)];
    let mut s = stack_with_budget(
        Config::scripted(vec![(CREATE, vec![status(0, 1, CREATE)]), (CANCEL, cancelled)]),
        5,
    );
    let create = op::LE_CREATE_CONNECTION;
    assert_eq!(s.connect(&CONNECT).err(), Some(HciError::Timeout(create)));
    assert_eq!(s.transport().pipes().commands[1].1, [0x0e, 0x20, 0]);
    assert_eq!(s.next_event(), Ok(None));
}

#[test]
fn a_cancel_after_the_link_came_up_is_disallowed_and_the_link_kept() {
    // The link's event may come before the cancel's answer or after it.
    let (up, no) = (connected(0, 0x41), complete(1, CANCEL, 0x0c, &[]));
    for late in [vec![no.clone(), up.clone()], vec![up, no]] {
        let cfg = Config::scripted(vec![(CREATE, vec![status(0, 1, CREATE)]), (CANCEL, late)]);
        let mut s = stack_with_budget(cfg, 5);
        assert_eq!(s.connect(&CONNECT).ok().map(|l| l.handle), Some(0x41));
        assert_eq!(s.next_event(), Ok(None));
    }
}

#[test]
fn a_cancel_refused_otherwise_is_its_status() {
    let cfg = Config::scripted(vec![
        (CREATE, vec![status(0, 1, CREATE)]),
        (CANCEL, vec![complete(1, CANCEL, 0x1f, &[])]),
    ]);
    let cancel = op::LE_CREATE_CONNECTION_CANCEL;
    let mut s = stack_with_budget(cfg, 5);
    assert_eq!(s.connect(&CONNECT).err(), Some(HciError::Status(cancel, 0x1f)));
}
