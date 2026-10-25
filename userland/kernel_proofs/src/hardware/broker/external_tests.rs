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

//! The buses behind an external-facing port come from its bridge
//! registers alone, and only numbers that describe buses below the port
//! are taken; a function is behind it on those buses, or is the port.

use super::external_range::{behind, bridge_buses, ExternalRange};

/// Primary, secondary, subordinate, latency timer: dword 0x18 as read.
fn buses(p: u8, s: u8, sub: u8) -> u32 {
    u32::from_le_bytes([p, s, sub, 0x40])
}

#[test]
fn a_bridge_below_its_port_gives_its_range() {
    assert_eq!(bridge_buses(0x01, buses(0, 5, 9), 0), Some((5, 9)));
    // multi-function bridge: bit 7 of the header type is not the type
    assert_eq!(bridge_buses(0x81, buses(0, 5, 5), 0), Some((5, 5)));
}

#[test]
fn bad_bus_numbers_and_endpoints_are_refused() {
    assert_eq!(bridge_buses(0x00, buses(0, 5, 9), 0), None);
    assert_eq!(bridge_buses(0x02, buses(0, 5, 9), 0), None);
    assert_eq!(bridge_buses(0x01, buses(1, 5, 9), 0), None);
    assert_eq!(bridge_buses(0x01, buses(0, 0, 0), 0), None);
    assert_eq!(bridge_buses(0x01, buses(4, 3, 9), 4), None);
    assert_eq!(bridge_buses(0x01, buses(0, 6, 5), 0), None);
    assert_eq!(bridge_buses(0x01, 0xFFFF_FFFF, 0xFF), None);
}

#[test]
fn behind_is_the_port_and_its_buses_in_its_segment() {
    let r = ExternalRange {
        segment: 0,
        port: (0, 0x1C, 4),
        secondary: 5,
        subordinate: 9,
        dma_property: true,
    };
    assert!(behind(&r, 0, (0, 0x1C, 4)));
    assert!(behind(&r, 0, (5, 0, 0)));
    assert!(behind(&r, 0, (9, 31, 7)));
    assert!(!behind(&r, 0, (4, 0, 0)));
    assert!(!behind(&r, 0, (10, 0, 0)));
    assert!(!behind(&r, 0, (0, 0x1C, 0)));
    assert!(!behind(&r, 1, (5, 0, 0)));
}
