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

//! The configuration walk's edges, by name: endpoint numbers, packet sizes
//! and interface kinds.

use super::{ConfigCase, KBD};
use crate::config_blob::{config, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;

fn with_endpoint(address: u8, size: u16) -> Vec<u8> {
    config().iface(0, HID, 1, 1).hid().ep(address, INTERRUPT, size, 10).build()
}

fn kind_with(subclass: u8, protocol: u8, size: u16) -> Vec<u8> {
    config().iface(0, HID, subclass, protocol).ep(0x81, INTERRUPT, size, 10).build()
}

#[test]
fn endpoint_and_packet_edges() {
    let report = |size| Ok(vec![(HidKind::Report, 0, 0x81, size)]);
    let cases: Vec<ConfigCase> = vec![
        ("endpoint 0 IN", with_endpoint(0x80, 8), Ok(vec![])),
        ("endpoint 1 IN", with_endpoint(0x81, 8), Ok(vec![KBD])),
        ("endpoint 15 IN", with_endpoint(0x8f, 8), Ok(vec![(HidKind::Keyboard, 0, 0x8f, 8)])),
        ("endpoint 1 OUT", with_endpoint(0x01, 8), Ok(vec![])),
        ("keyboard packet 0", with_endpoint(0x81, 0), Ok(vec![])),
        ("keyboard packet 7", with_endpoint(0x81, 7), Ok(vec![])),
        (
            "keyboard packet 1024",
            with_endpoint(0x81, 1024),
            Ok(vec![(HidKind::Keyboard, 0, 0x81, 1024)]),
        ),
        ("keyboard packet 1025", with_endpoint(0x81, 1025), Ok(vec![])),
        ("keyboard packet 0x7ff", with_endpoint(0x81, 0x7ff), Ok(vec![])),
        ("keyboard packet 8, high bandwidth bits", with_endpoint(0x81, 0x1808), Ok(vec![KBD])),
        ("mouse packet 2", kind_with(1, 2, 2), Ok(vec![])),
        ("mouse packet 3", kind_with(1, 2, 3), Ok(vec![(HidKind::Mouse, 0, 0x81, 3)])),
        ("report protocol packet 0", kind_with(0, 0, 0), Ok(vec![])),
        ("report protocol packet 1", kind_with(0, 0, 1), report(1)),
        ("report protocol packet 1024", kind_with(0, 0, 1024), report(1024)),
        ("report protocol packet 1025", kind_with(0, 0, 1025), Ok(vec![])),
        ("boot subclass, protocol 0", kind_with(1, 0, 8), Ok(vec![])),
        ("boot subclass, protocol 3", kind_with(1, 3, 8), Ok(vec![])),
    ];
    for (name, raw, want) in cases {
        let got = hid_bindings(&raw).map(|b| summaries(&b));
        assert_eq!(got, want, "{name}");
    }
}
