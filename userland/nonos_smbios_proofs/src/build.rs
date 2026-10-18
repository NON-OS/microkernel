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

//! Tables built byte by byte the way DSP0134 lays them out, for the
//! proofs: one structure from its type, handle, formatted fields and
//! strings; and QEMU's machine, as `-machine q35` describes itself.

pub fn structure(kind: u8, handle: u16, fields: &[u8], strings: &[&str]) -> Vec<u8> {
    let mut v = vec![kind, (4 + fields.len()) as u8];
    v.extend_from_slice(&handle.to_le_bytes());
    v.extend_from_slice(fields);
    for s in strings {
        v.extend_from_slice(s.as_bytes());
        v.push(0);
    }
    if strings.is_empty() {
        v.push(0);
    }
    v.push(0);
    v
}

pub fn qemu() -> Vec<u8> {
    let bios = structure(
        0,
        0,
        &[1, 2, 0, 0xE8, 3, 0],
        &["EFI Development Kit II / OVMF", "0.0.0", "02/06/2015"],
    );
    let sys = structure(
        1,
        0x100,
        &[1, 2, 3, 0],
        &["QEMU", "Standard PC (Q35 + ICH9, 2009)", "pc-q35-8.2"],
    );
    let chassis = structure(3, 0x300, &[1, 0x01, 2], &["QEMU", "pc-q35-8.2"]);
    [bios, sys, chassis, structure(127, 0x7F00, &[], &[])].concat()
}
