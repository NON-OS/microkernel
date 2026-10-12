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

//! Event packets (Vol 4, Part E, 5.4.4) and the events every command and
//! link uses, laid out by hand from Vol 4, Part E, 7.7.

use nonos_bt_wire::hci::opcode::{LE_CREATE_CONNECTION, RESET};
use nonos_bt_wire::hci::reports::completed_packets;
use nonos_bt_wire::hci::{event_code as ev, parse_event};
use nonos_bt_wire::hci::{CommandComplete, CommandStatus, DisconnectionComplete};
use nonos_bt_wire::WireError;

#[test]
fn command_complete_for_reset() {
    let e = parse_event(&[0x0e, 0x04, 0x01, 0x03, 0x0c, 0x00]).ok().unwrap();
    assert_eq!(e.code, ev::COMMAND_COMPLETE);
    let cc = CommandComplete::parse(e.params).ok().unwrap();
    assert_eq!((cc.num_packets, cc.opcode, cc.ret), (1, RESET, &[0u8][..]));
}

#[test]
fn command_status_and_disconnection_complete() {
    let cs = CommandStatus::parse(&[0x00, 0x01, 0x0d, 0x20]).ok().unwrap();
    assert_eq!((cs.status, cs.num_packets, cs.opcode), (0, 1, LE_CREATE_CONNECTION));
    assert!(CommandStatus::parse(&[0, 1, 0x0d, 0x20, 0]).is_err());
    let d = DisconnectionComplete::parse(&[0x00, 0x40, 0x00, 0x13]).ok().unwrap();
    assert_eq!((d.status, d.handle, d.reason), (0, 0x0040, 0x13));
}

#[test]
fn completed_packets_pairs() {
    let p = [2, 0x40, 0x00, 3, 0x00, 0x41, 0x00, 1, 0x00];
    assert_eq!(completed_packets(&p), Ok(vec![(0x40, 3), (0x41, 1)]));
    assert_eq!(completed_packets(&p[..7]), Err(WireError::Short));
    assert_eq!(completed_packets(&[0, 1]), Err(WireError::Length));
}

#[test]
fn an_event_whose_length_disagrees_is_refused() {
    assert_eq!(parse_event(&[0x0e]).err(), Some(WireError::Short));
    assert_eq!(parse_event(&[0x0e, 0x04, 0x01]).err(), Some(WireError::Short));
    assert_eq!(parse_event(&[0x0e, 0x00, 0x01]).err(), Some(WireError::Length));
    assert_eq!(parse_event(&[0x10, 0x01, 0x07]).map(|e| e.code), Ok(ev::HARDWARE_ERROR));
}
