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

//! PCCT subspaces as firmware lays them out (offsets recalled from ACPI
//! 6.5, 14.1, unverified), each register a memory GAS at a given address.

use super::build::*;

fn gas(at: u64) -> Vec<u8> {
    [vec![0, 32, 0, 3], u64le(at)].concat()
}

/// A register and its two masks.
fn reg(at: u64, preserve: u64, write: u64) -> Vec<u8> {
    [gas(at), u64le(preserve), u64le(write)].concat()
}

/// Type 0, 1 or 2; type 2 adds the acknowledge register.
pub fn generic(kind: u8, base: u64) -> Vec<u8> {
    let len = if kind == 2 { 90 } else { 62 };
    let mut s = [vec![kind, len], u32le(if kind > 0 { 40 } else { 0 }), vec![1, 0]].concat();
    s.extend([u64le(base), u64le(0x1000), reg(0xFE10_0000, 0xFFFF_0000, 1)].concat());
    s.extend([u32le(50), u32le(1000), u16le(10)].concat());
    if kind == 2 {
        s.extend(reg(0xFE10_0010, 0, 2));
    }
    s
}

/// Type 3 or 4, the extended channels.
pub fn extended(kind: u8) -> Vec<u8> {
    let mut s =
        [vec![kind, 164], u32le(41), vec![3, 0], u64le(0x8000_0000), u32le(0x2000)].concat();
    s.extend([reg(0xFE20_0000, 0, 1), u32le(60), u32le(500), u32le(20)].concat());
    s.extend([reg(0xFE20_0008, 0, 1), u64le(0)].concat());
    s.extend([gas(0xFE20_0010), u64le(1), reg(0xFE20_0014, 0, 1)].concat());
    s.extend([gas(0xFE20_0018), u64le(4)].concat());
    s
}

/// Type 5, register based.
pub fn registers() -> Vec<u8> {
    let mut s = [vec![5, 96], u16le(1), u64le(0x9000_0000), u64le(0x400)].concat();
    s.extend([reg(0xFE30_0000, 0, 1), gas(0xFE30_0004), u64le(1)].concat());
    s.extend([gas(0xFE30_0008), u64le(2), u32le(70), u32le(30)].concat());
    s
}

pub fn table(flags: u32, subspaces: &[Vec<u8>]) -> Vec<u8> {
    sdt(b"PCCT", 1, &[u32le(flags), u64le(0), subspaces.concat()].concat())
}
