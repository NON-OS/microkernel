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

use super::sdhci::{encode_identify, MEDIUM_EMMC, OP_IDENTIFY};
use super::wire_identity_parse::{parse_wire_identity, OP_WIRE_IDENTIFY, WIRE_IDENTIFY_LEN};

#[test]
fn the_installer_names_the_part_driver_sdhci0_describes() {
    let body = encode_identify(b"SK hynix HAG4a2", b"0102ABCD", MEDIUM_EMMC).expect("valid");
    assert_eq!((OP_WIRE_IDENTIFY, body.len()), (OP_IDENTIFY, WIRE_IDENTIFY_LEN));
    let id = parse_wire_identity(&body).expect("taken");
    assert_eq!((id.model_str(), id.serial_str(), id.emmc), ("SK hynix HAG4a2", "0102ABCD", true));
}

#[test]
fn a_body_breaking_the_wires_rules_names_nothing() {
    let good = encode_identify(b"Samsung 8GTF4R", b"DEADBEEF", MEDIUM_EMMC).expect("valid");
    let mut short = good.to_vec();
    short.pop();
    assert!(parse_wire_identity(&short).is_none());
    let mut b = good;
    b[3] = 1;
    assert!(parse_wire_identity(&b).is_none(), "reserved");
    let mut b = good;
    b[0] = 41;
    assert!(parse_wire_identity(&b).is_none(), "model length");
    let mut b = good;
    b[44] = 0x07;
    assert!(parse_wire_identity(&b).is_none(), "control byte in the serial");
    let mut b = good;
    b[63] = b'Z';
    assert!(parse_wire_identity(&b).is_none(), "serial padding");
    let mut b = good;
    b[2] = 2;
    assert!(!parse_wire_identity(&b).expect("an SD card").emmc);
}
