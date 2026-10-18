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

//! SRAT entries as firmware lays them out (the packed structs in the x86
//! tables), each kind with its enabled flag, and the table around them.

use super::build::*;

pub fn apic(apic_id: u8, domain: u32, on: bool) -> Vec<u8> {
    let d = domain.to_le_bytes();
    let rest = [vec![d[0], apic_id], u32le(u32::from(on)), vec![0, d[1], d[2], d[3]], u32le(0)];
    structure(0, &rest.concat())
}

pub fn memory(domain: u32, base: u64, len: u64, flags: u32) -> Vec<u8> {
    let rest = [u32le(domain), u16le(0), u64le(base), u64le(len), u32le(0), u32le(flags), u64le(0)];
    structure(1, &rest.concat())
}

pub fn x2apic(id: u32, domain: u32, on: bool) -> Vec<u8> {
    let rest = [u16le(0), u32le(domain), u32le(id), u32le(u32::from(on)), u32le(0), u32le(0)];
    structure(2, &rest.concat())
}

pub fn gicc(uid: u32, domain: u32, on: bool) -> Vec<u8> {
    structure(3, &[u32le(domain), u32le(uid), u32le(u32::from(on)), u32le(0)].concat())
}

/// Handle type and its 16 bytes.
pub fn initiator(kind: u8, domain: u32, handle: [u8; 16]) -> Vec<u8> {
    let rest = [vec![0, kind], u32le(domain), handle.to_vec(), u32le(1), u32le(0)];
    structure(5, &rest.concat())
}

pub fn srat(entries: &[Vec<u8>]) -> Vec<u8> {
    sdt(b"SRAT", 3, &[u32le(1), u64le(0), entries.concat()].concat())
}

pub fn slit(n: u64, matrix: &[u8]) -> Vec<u8> {
    sdt(b"SLIT", 1, &[u64le(n), matrix.to_vec()].concat())
}
