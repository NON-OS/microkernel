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

//! LE Connection Update (Vol 4, Part E, 7.8.18), sent once a peripheral's
//! request for new timing is accepted.

use crate::sim::config::Config;
use crate::sim::events::status;
use crate::sim::stack::stack;
use nonos_bt_hci::host::{connection_update_params, DEFAULT_LINK};
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::l2cap::ConnParams;
use nonos_bt_wire::WireError;

const UPDATE: u16 = op::LE_CONNECTION_UPDATE.0;

#[test]
fn the_update_is_laid_out_and_taken_on_command_status() {
    let mut s = stack(Config::scripted(vec![(UPDATE, vec![status(0, 1, UPDATE)])]));
    assert_eq!(s.update_link(0x40, &DEFAULT_LINK), Ok(()));
    let want = [0x13, 0x20, 14, 0x40, 0, 0x10, 0, 0x20, 0, 0, 0, 0x90, 0x01, 0, 0, 0, 0];
    assert_eq!(s.transport().pipes().commands[0].1, want);
}

#[test]
fn a_refused_update_is_its_status() {
    let mut s = stack(Config::scripted(vec![(UPDATE, vec![status(0x02, 1, UPDATE)])]));
    let e = s.update_link(0x40, &DEFAULT_LINK);
    assert_eq!(e, Err(HciError::Status(op::LE_CONNECTION_UPDATE, 0x02)));
}

#[test]
fn bad_timing_or_handle_is_refused_unsent() {
    let slow = ConnParams { latency: 0x01f3, ..DEFAULT_LINK };
    let reserved = HciError::Wire(WireError::Reserved);
    assert_eq!(connection_update_params(0x40, &slow), Err(reserved));
    assert_eq!(connection_update_params(0x0f00, &DEFAULT_LINK), Err(reserved));
    let mut s = stack(Config::default());
    assert_eq!(s.update_link(0x40, &slow), Err(reserved));
    assert!(s.transport().pipes().commands.is_empty());
}
