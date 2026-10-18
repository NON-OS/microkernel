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

//! Command packets (Vol 4, Part E, 5.4.1) and opcodes (Vol 4, Part E, 7).

use nonos_bt_wire::hci::opcode::{self, Opcode};
use nonos_bt_wire::hci::{command, parse_command};
use nonos_bt_wire::WireError;

#[test]
fn reset_is_opcode_0x0c03_with_no_parameters() {
    assert_eq!((opcode::RESET.ogf(), opcode::RESET.ocf()), (0x03, 0x0003));
    assert_eq!(command(opcode::RESET, &[]), Ok(vec![0x03, 0x0c, 0x00]));
}

#[test]
fn le_commands_are_ogf_8() {
    assert_eq!(opcode::LE_SET_SCAN_ENABLE.0, 0x200c);
    assert_eq!(opcode::LE_CREATE_CONNECTION.0, 0x200d);
    assert_eq!(opcode::LE_LTK_REQUEST_REPLY.0, 0x201a);
    assert_eq!(opcode::LE_CONNECTION_UPDATE.0, 0x2013);
    assert_eq!(opcode::LE_SET_RANDOM_ADDRESS.0, 0x2005);
    assert_eq!(opcode::READ_BD_ADDR.0, 0x1009);
    assert_eq!(opcode::READ_BUFFER_SIZE.0, 0x1005);
    assert_eq!(opcode::DISCONNECT.0, 0x0406);
    assert_eq!(Opcode::new(0x3f, 0x3ff).0, 0xffff);
}

#[test]
fn a_command_round_trips_and_length_is_checked() {
    let pkt = command(opcode::LE_SET_SCAN_ENABLE, &[1, 0]).unwrap();
    assert_eq!(pkt, [0x0c, 0x20, 2, 1, 0]);
    assert_eq!(parse_command(&pkt), Ok((opcode::LE_SET_SCAN_ENABLE, &[1u8, 0][..])));
    assert_eq!(parse_command(&pkt[..4]), Err(WireError::Short));
    assert_eq!(parse_command(&[pkt.as_slice(), &[0]].concat()), Err(WireError::Length));
    assert_eq!(command(opcode::RESET, &[0; 256]), Err(WireError::TooLong));
    assert_eq!(command(opcode::RESET, &[0; 255]).map(|p| p.len()), Ok(258));
}
