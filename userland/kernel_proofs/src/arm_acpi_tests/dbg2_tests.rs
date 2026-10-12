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

//! DBG2 (recalled, unverified): a serial and a USB debug port read; a
//! structure past the table and a path inside the header refused.

use super::build::*;
use crate::arm_acpi::dbg2::{dbg2, PORT_SERIAL};
use crate::arm_acpi::sdt::TableError;

fn device(port: u16, sub: u16, regs: &[(u8, u64, u32)], path: &[u8]) -> Vec<u8> {
    let n = regs.len();
    let (regs_at, sizes_at, ns_at) = (22, 22 + 12 * n, 22 + 16 * n);
    let len = ns_at + path.len();
    let mut d = [vec![0], u16le(len as u16), vec![n as u8], u16le(path.len() as u16)].concat();
    d.extend([u16le(ns_at as u16), u16le(0), u16le(0), u16le(port), u16le(sub), u16le(0)].concat());
    d.extend([u16le(regs_at as u16), u16le(sizes_at as u16)].concat());
    for (space, at, _) in regs {
        d.extend([vec![*space, 8, 0, 1], u64le(*at)].concat());
    }
    for (_, _, size) in regs {
        d.extend(u32le(*size));
    }
    [d, path.to_vec()].concat()
}

fn table(devices: &[Vec<u8>], count: u32) -> Vec<u8> {
    sdt(b"DBG2", 0, &[u32le(44), u32le(count), devices.concat()].concat())
}

#[test]
fn a_serial_and_a_usb_port() {
    let com = device(PORT_SERIAL, 0, &[(1, 0x2F8, 8)], b"\\_SB.COM2\0");
    let usb = device(0x8002, 0, &[(0, 0xFE00_0000, 0x1000), (0, 0xFE01_0000, 0x100)], b"");
    let d = dbg2(&table(&[com, usb], 2)).unwrap();
    assert_eq!(d.len(), 2);
    assert_eq!(
        (d[0].port_type, d[0].subtype, d[0].namespace.as_slice()),
        (0x8000, 0, &b"\\_SB.COM2"[..])
    );
    assert_eq!(
        (d[0].registers[0].0.space, d[0].registers[0].0.address, d[0].registers[0].1),
        (1, 0x2F8, 8)
    );
    assert_eq!(d[1].registers.len(), 2);
    assert_eq!(d[1].registers[1].1, 0x100);
}

#[test]
fn bad_structures() {
    let com = device(PORT_SERIAL, 0, &[(1, 0x2F8, 8)], b"\\X\0");
    assert_eq!(
        dbg2(&table(std::slice::from_ref(&com), 2)),
        Err(TableError::Structure(44 + com.len()))
    );
    let mut inside = com.clone();
    inside[6..8].copy_from_slice(&u16le(4));
    assert_eq!(dbg2(&table(&[inside], 1)), Err(TableError::Structure(44)));
    let mut t = table(&[com], 1);
    t[9] ^= 1;
    assert_eq!(dbg2(&t), Err(TableError::Checksum));
}
