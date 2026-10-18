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

//! The CCI, the commands and the replies, field by field.

use crate::capability::{connector_capability, connectors};
use crate::command::*;
use crate::layout::Cci;
use crate::status::status;

#[test]
fn every_cci_field() {
    let c = Cci::decode(1 << 31 | 1 << 29 | 1 << 27 | 9 << 8 | 5 << 1);
    assert_eq!((c.connector, c.data_len), (5, 9));
    assert!(c.command_completed && c.ack_command && c.reset_completed);
    assert!(!(c.busy || c.error || c.not_supported || c.cancel_completed));
    let all = Cci::decode(0x7E00_0000);
    assert!(all.busy && all.error && all.not_supported && all.cancel_completed);
    assert_eq!(Cci::decode(0xFF).connector, 0x7F, "seven bits above bit 0");
}

#[test]
fn commands_as_control_words() {
    assert_eq!(reset(), 0x01);
    assert_eq!(connector_status(3), 0x12 | 3 << 16);
    assert_eq!(connector_status(0xFF), 0x12 | 0x7F << 16, "seven bits name a connector");
    assert_eq!(ack(true, false), 0x04 | 2 << 16);
    assert_eq!(ack(true, true), 0x04 | 3 << 16);
    assert_eq!(notifications(NOTIFY), 0x05 | 0xDA67 << 16);
}

#[test]
fn replies_field_by_field() {
    let mut b = [0u8; 9];
    b[0..2].copy_from_slice(&0x4000u16.to_le_bytes());
    let w: u16 = 3 | 1 << 3 | 0x81 << 5 | 2 << 13; // bit 4 clear: sinking
    b[2..4].copy_from_slice(&w.to_le_bytes());
    b[4..8].copy_from_slice(&0x1234_5678u32.to_le_bytes());
    b[8] = 0b1101;
    let s = status(&b).unwrap();
    assert_eq!((s.change, s.power_mode, s.connected, s.provider), (0x4000, 3, true, false));
    assert_eq!((s.partner_flags, s.partner_type, s.rdo, s.charging), (0x81, 2, 0x1234_5678, 1));
    assert!(s.charger(), "connected and sinking");
    assert_eq!(status(&b[..8]), None, "short");
    assert_eq!(connectors(&[0, 0, 0, 0, 0x83]), Some(3));
    assert_eq!(connectors(&[0; 4]), None);
    let cap = connector_capability(&[0b100, 0b10]).unwrap();
    assert!(cap.drp && !cap.provider && cap.consumer);
}
