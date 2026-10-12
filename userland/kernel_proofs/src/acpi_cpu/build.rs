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

//! Records as the capsule writes them, built field by field.

use super::cpu_layout::*;

pub fn reg(kind: u8, space: u8, width: u8, value: u64) -> Vec<u8> {
    let mut b = vec![kind, space, width, 0, if kind == REGISTER { 4 } else { 0 }, 0, 0, 0];
    b.extend_from_slice(&value.to_le_bytes());
    b
}

pub fn header(np: u8, nc: u8, flags: u8, uid: u64, ppc: u64) -> Vec<u8> {
    let mut b = vec![VERSION, np, nc, flags, 0, 0, 0, 0];
    b.extend_from_slice(&uid.to_le_bytes());
    b.extend_from_slice(&ppc.to_le_bytes());
    b
}

pub fn words(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// A laptop core: _PCT in fixed hardware, two P-states, _PPC 1, C1 by
/// fixed hardware and C2 by an I/O port read, and a _CPC revision 3.
pub fn laptop() -> Vec<u8> {
    let mut b = header(2, 2, HAS_PCT | HAS_PPC | HAS_CPC, 4, 1);
    b.extend(reg(REGISTER, 0x7F, 64, 0x199));
    b.extend(reg(REGISTER, 0x7F, 64, 0x198));
    b.extend(words(&[2400, 15000, 10, 10, 0x1800, 0x1800]));
    b.extend(words(&[800, 4000, 10, 10, 0x800, 0x800]));
    b.extend(reg(REGISTER, 0x7F, 1, 0));
    b.extend(words(&[1, 1, 1000]));
    b.extend(reg(REGISTER, 0x01, 8, 0x414));
    b.extend(words(&[2, 50, 500]));
    b.extend([3, 0, 0, 0, 0, 0, 0, 0]);
    for i in 0..CPC_FIELDS as u64 {
        b.extend(match i {
            5 => reg(REGISTER, 0x0A, 32, 0x14),
            4 => reg(ABSENT, 0, 0, 0),
            i => reg(INTEGER, 0, 0, 100 + i),
        });
    }
    b
}
