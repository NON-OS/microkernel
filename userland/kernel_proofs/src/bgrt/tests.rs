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

//! BGRT as ACPI 6.5 5.2.23 lays it out (recalled, unverified): the logo's
//! place, status bits and orientation; a wrong version, no address, short.

use super::bgrt_entry::{bgrt, Bgrt};

fn table(version: u16, status: u8, address: u64) -> Vec<u8> {
    let mut t = b"BGRT".to_vec();
    t.extend_from_slice(&[0; 32]);
    t.extend_from_slice(&version.to_le_bytes());
    t.extend_from_slice(&[status, 0]);
    t.extend_from_slice(&address.to_le_bytes());
    t.extend_from_slice(&120u32.to_le_bytes());
    t.extend_from_slice(&340u32.to_le_bytes());
    t
}

#[test]
fn the_logo_and_its_place() {
    let b = bgrt(&table(1, 0b011, 0x7E00_0000)).unwrap();
    let want = Bgrt {
        displayed: true,
        orientation: 1,
        image_type: 0,
        address: 0x7E00_0000,
        x: 120,
        y: 340,
    };
    assert_eq!(b, want);
    assert_eq!(bgrt(&table(1, 0b110, 0x1000)).unwrap().orientation, 3);
    assert!(!bgrt(&table(1, 0b110, 0x1000)).unwrap().displayed);
}

#[test]
fn refused_tables() {
    assert_eq!(bgrt(&table(2, 1, 0x1000)), None, "version 2");
    assert_eq!(bgrt(&table(1, 1, 0)), None, "no image");
    assert_eq!(bgrt(&table(1, 1, 0x1000)[..55]), None, "short");
}
