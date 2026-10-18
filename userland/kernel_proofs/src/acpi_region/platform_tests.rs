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

//! Platform device records: decoded only when every field holds.

use super::platform_decode::decode;
use super::platform_wire::{GPIO, HAS_IRQ, I2C};

pub(super) fn record(class: u8, hid: &[u8], windows: &[(u64, u64)], path: &[u8]) -> Vec<u8> {
    let mut b = vec![1, class, windows.len() as u8, HAS_IRQ, 0, 0, 0, 0];
    let mut h = [0u8; 8];
    h[..hid.len()].copy_from_slice(hid);
    b.extend_from_slice(&h);
    b.extend_from_slice(&7u64.to_le_bytes());
    b.extend_from_slice(&33u32.to_le_bytes());
    b.extend_from_slice(&[0; 4]);
    windows.iter().for_each(|(base, len)| {
        [base, len].iter().for_each(|v| b.extend_from_slice(&v.to_le_bytes()))
    });
    b.push(path.len() as u8);
    b.extend_from_slice(path);
    b
}

#[test]
fn a_well_formed_record_decodes() {
    let p = decode(&record(I2C, b"AMDI0010", &[(0xFEDC_2000, 0x1000)], b"\\_SB.I2CA")).unwrap();
    assert_eq!((p.class, &p.hid, p.uid, p.gsiv), (I2C, b"AMDI0010", 7, 33));
    assert_eq!(
        (p.windows.as_slice(), p.path.as_slice()),
        (&[(0xFEDC_2000, 0x1000)][..], &b"\\_SB.I2CA"[..])
    );
    let g = decode(&record(
        GPIO,
        b"INT33FC",
        &[(0xFED0_C000, 0x1000), (0xFED0_D000, 0x1000)],
        b"\\_SB.GPO0",
    ));
    assert_eq!(g.map(|g| g.windows.len()), Some(2));
}

#[test]
fn every_field_is_checked() {
    let ok = record(I2C, b"INT3432", &[(0xFE10_4000, 0x1000)], b"\\_SB.I2C1");
    let mutate = |at: usize, v: u8| {
        let mut b = ok.clone();
        b[at] = v;
        decode(&b)
    };
    assert_eq!(mutate(0, 2), None, "version");
    assert_eq!(mutate(1, 9), None, "class");
    assert_eq!(mutate(2, 0), None, "no window");
    assert_eq!(mutate(5, 1), None, "reserved byte");
    assert_eq!(mutate(8, b'-'), None, "a _HID of letters and digits");
    assert_eq!(mutate(12, 0), None, "NUL padding, then nothing");
    assert_eq!(mutate(49, b'/'), None, "the path starts with a backslash");
    assert_eq!(decode(&[ok.clone(), vec![0]].concat()), None, "a trailing byte");
    assert_eq!(decode(&ok[..ok.len() - 1]), None, "cut short");
    assert_eq!(decode(&record(I2C, b"", &[(1, 1)], b"\\X")), None, "no _HID");
}
