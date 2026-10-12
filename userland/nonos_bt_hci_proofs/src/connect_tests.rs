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
//! Opening a link as central: LE Create Connection (Vol 4, Part E,
//! 7.8.12) and LE Connection Complete (7.7.65.1).

use crate::sim::config::Config;
use crate::sim::events::{adv_report, status};
use crate::sim::link_events::{connected, CONNECT, PEER};
use crate::sim::stack::stack;
use nonos_bt_hci::host::{create_connection_params, Connect, Scan, DEFAULT_LINK};
use nonos_bt_hci::HciError;
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::l2cap::ConnParams;
use nonos_bt_wire::WireError;

const CREATE: u16 = op::LE_CREATE_CONNECTION.0;
const C: Connect = CONNECT;

#[test]
fn the_parameters_are_sent_as_laid_out() {
    let mut want = vec![0x0d, 0x20, 25, 0x60, 0x00, 0x30, 0x00, 0x00, 0x01];
    want.extend_from_slice(&PEER);
    want.extend_from_slice(&[0x00, 0x10, 0x00, 0x20, 0x00, 0x00, 0x00, 0x90, 0x01, 0, 0, 0, 0]);
    let mut s =
        stack(Config::scripted(vec![(CREATE, vec![status(0, 1, CREATE), connected(0, 0x40)])]));
    let link = s.connect(&C).ok().unwrap();
    assert_eq!((link.handle, link.peer, link.peer_type), (0x40, BdAddr(PEER), AddrType::Random));
    assert_eq!(s.transport().pipes().commands[0].1, want);
}

#[test]
fn timing_outside_the_ranges_is_refused_unsent() {
    let long_window = Connect { scan: Scan { interval: 0x30, window: 0x31 }, ..C };
    // 500 skipped events at 40 ms outlast a 4 s supervision timeout.
    let too_slow = ConnParams { latency: 0x01f3, ..DEFAULT_LINK };
    let mut s = stack(Config::default());
    for c in [long_window, Connect { link: too_slow, ..C }] {
        assert_eq!(create_connection_params(&c), Err(HciError::Wire(WireError::Reserved)));
        assert_eq!(s.connect(&c).err(), Some(HciError::Wire(WireError::Reserved)));
    }
    assert!(s.transport().pipes().commands.is_empty());
}

#[test]
fn reports_met_while_connecting_stay_queued_and_a_failure_is_its_status() {
    let report = adv_report(9, &[]);
    let events = vec![status(0, 1, CREATE), report.clone(), connected(0x3e, 0)];
    let mut s = stack(Config::scripted(vec![(CREATE, events)]));
    let create = op::LE_CREATE_CONNECTION;
    assert_eq!(s.connect(&C).err(), Some(HciError::Status(create, 0x3e)));
    assert_eq!(s.next_event(), Ok(Some(report)));
    assert_eq!(s.next_event(), Ok(None));
}
