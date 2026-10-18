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

//! SPCR by revision: the port, its interrupt and rate; the clock from
//! revision 3; the precise rate and namespace path from revision 4.

use super::build::*;
use crate::arm_acpi::sdt::TableError;
use crate::arm_acpi::spcr::spcr;
use crate::arm_acpi::spcr::Interface;

pub fn table(rev: u8, pci: bool, path: &[u8]) -> Vec<u8> {
    let mut b = [vec![0x03, 0, 0, 0], vec![0, 32, 0, 3], u64le(0x0900_0000)].concat();
    b.extend([vec![8, 0], u32le(33), vec![7, 0, 1, 0, 0, 0]].concat());
    b.extend(if pci { u16le(0x1234) } else { u16le(0xFFFF) });
    b.extend([u16le(0x8086), vec![2, 3, 4], u32le(0), vec![1]].concat());
    b.extend(u32le(if rev >= 3 { 24_000_000 } else { 0 }));
    if rev >= 4 {
        b.extend([u32le(1_500_000), u16le(path.len() as u16), u16le(88), path.to_vec()].concat());
    }
    sdt(b"SPCR", rev, &b)
}

#[test]
fn each_revision_read() {
    let s = spcr(&table(2, false, b"")).unwrap();
    assert_eq!(
        (s.interface, s.base.address, s.base.bit_width),
        (Interface::Pl011, 0x0900_0000, 32)
    );
    assert_eq!((s.interrupt_type, s.gsiv, s.baud, s.stop_bits), (8, 33, Some(115_200), 1));
    assert_eq!((s.pci, s.clock_hz, s.precise_baud, s.namespace), (None, None, None, None));
    let s = spcr(&table(3, true, b"")).unwrap();
    assert_eq!((s.pci, s.clock_hz), (Some((1, 2, 3, 4)), Some(24_000_000)));
    let s = spcr(&table(4, false, b"\\_SB.COM0\0")).unwrap();
    assert_eq!(
        (s.precise_baud, s.namespace.as_deref()),
        (Some(1_500_000), Some(&b"\\_SB.COM0"[..]))
    );
}

#[test]
fn a_namespace_path_outside_the_table() {
    let mut t = table(4, false, b"\\X\0");
    t[84..86].copy_from_slice(&u16le(40));
    seal(&mut t);
    assert_eq!(spcr(&t), Err(TableError::Structure(88)));
    let mut t = table(4, false, b"\\X\0");
    t[86..88].copy_from_slice(&u16le(20));
    seal(&mut t);
    assert_eq!(spcr(&t), Err(TableError::Structure(20)), "inside the fixed part");
    assert_eq!(spcr(&table(4, false, b"")[..84]), Err(TableError::Length));
}
