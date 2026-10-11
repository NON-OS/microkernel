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

//! The CONTROLLER_INFO and PORT_LIST payload layouts.

use crate::emmc::info::*;
use crate::emmc::sdhci::{Caps, Snapshot};

#[test]
fn controller_info_layout() {
    let caps = Caps { caps: 0x1462_c8b2, caps1: 0x0000_0807, version: 0x1002 };
    let snap = Snapshot {
        present: 0x1ff_0000,
        host_control: 0x36,
        power: 0x0b,
        clock: 0x0207,
        int_status: 0,
    };
    let c = controller_info(&caps, &snap);
    assert_eq!(u32::from_le_bytes(c[0..4].try_into().unwrap()), 0x1462_c8b2);
    assert_eq!(u32::from_le_bytes(c[4..8].try_into().unwrap()), 0x0207_0b36);
    assert_eq!(u32::from_le_bytes(c[8..12].try_into().unwrap()), 1);
    assert_eq!(u32::from_le_bytes(c[12..16].try_into().unwrap()), 0x1002);
    assert_eq!(u32::from_le_bytes(c[16..20].try_into().unwrap()), 0x0807);
    assert_eq!(&c[20..24], &[1, 0, 0, 0]);
}

#[test]
fn port_entry_layout() {
    let snap = Snapshot {
        present: 0x01ff_0000,
        host_control: 0,
        power: 0,
        clock: 0,
        int_status: 0x0020_8002,
    };
    let p = port_entry(true, &snap, 0xc0ff_8080, 0x900);
    assert_eq!(&p[0..4], &[0, 1, 1, PORT_KIND_EMMC]);
    assert_eq!(u32::from_le_bytes(p[4..8].try_into().unwrap()), 0x01ff_0000);
    assert_eq!(u32::from_le_bytes(p[8..12].try_into().unwrap()), 0xc0ff_8080);
    assert_eq!(u32::from_le_bytes(p[12..16].try_into().unwrap()), 0x0020_8002);
    assert_eq!(u32::from_le_bytes(p[16..20].try_into().unwrap()), 0x900);
    assert_eq!(&p[20..24], &[0; 4]);
    assert_eq!(u32::from_le_bytes(p[24..28].try_into().unwrap()), 0x20);
    assert_eq!(&p[28..36], &[0; 8]);
    assert_eq!(port_entry(false, &snap, 0, 0)[2], 0);
}
