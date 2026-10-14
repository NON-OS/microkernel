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

//! The reach narrowing on a client's read, against every head a read can
//! come back with: only the declared descriptor register, written whole,
//! and an HID descriptor's head read back, narrow it, and then to the
//! address read alone.

use crate::transaction::Reach;

const DESC: [u8; 4] = [30, 0, 0x00, 0x01];

fn wide() -> Reach {
    Reach::declared([(0x2C, 0x0001), (0x15, 0x0020)])
}

fn reached(r: &Reach) -> Vec<u8> {
    (0..=u8::MAX).filter(|&a| r.allows(a)).collect()
}

#[test]
fn each_address_keeps_the_register_its_declaration_names() {
    let r = wide();
    assert_eq!((r.desc_reg(0x2C), r.desc_reg(0x15)), (Some(0x0001), Some(0x0020)));
    assert_eq!((r.desc_reg(0x10), r.desc_reg(0x00), r.desc_reg(0xAC)), (None, None, None));
}

#[test]
fn a_descriptor_read_at_the_declared_register_narrows_to_that_address() {
    let r = wide().after_read(0x15, &[0x20, 0x00], &DESC);
    assert_eq!(reached(&r), [0x15]);
    assert_eq!(r.desc_reg(0x15), Some(0x0020));
    let again = r.after_read(0x15, &[0x20, 0x00], &[DESC.as_slice(), &[0; 26]].concat());
    assert_eq!(reached(&again), [0x15], "a second read changes nothing");
}

#[test]
fn nothing_but_that_read_narrows_and_nothing_widens() {
    let reg = [0x01, 0x00];
    let cases: [(u8, &[u8], &[u8]); 9] = [
        (0x2C, &[0x20, 0x00], &DESC),
        (0x2C, &[0x01], &DESC),
        (0x2C, &[0x01, 0x00, 0x00], &DESC),
        (0x2C, &[], &DESC),
        (0x2C, &reg, &DESC[..3]),
        (0x2C, &reg, &[]),
        (0x2C, &reg, &[31, 0, 0x00, 0x01]),
        (0x2C, &reg, &[30, 0, 0x01, 0x01]),
        (0x10, &reg, &DESC),
    ];
    for (addr, write, read) in cases {
        let r = wide().after_read(addr, write, read);
        assert_eq!(reached(&r), [0x15, 0x2C], "{addr:#04x} {write:02X?} {read:02X?}");
    }
    let narrow = wide().after_read(0x2C, &reg, &DESC);
    assert_eq!(reached(&narrow.after_read(0x15, &[0x20, 0x00], &DESC)), [0x2C]);
}
