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

//! Idle levels as the capsule appends them to a MkAcpiCpu record.

use super::build::{levels_header, reg, words};
use super::cpu_layout::*;

pub fn level(kind: u8, uid: u64, id: u64, n: u8) -> Vec<u8> {
    let mut b = uid.to_le_bytes().to_vec();
    b.extend(id.to_le_bytes());
    b.extend([kind, n, 0, 0, 0, 0, 0, 0]);
    b
}

/// A state entered through `entry`, allowing the parent's state `parent`,
/// with a residency counter in memory and no usage counter.
pub fn state(latency: u32, parent: u32, entry: Vec<u8>) -> Vec<u8> {
    let mut b = words(&[latency * 5, latency, 1, parent]);
    b.extend(0u64.to_le_bytes());
    b.extend(1_000_000u64.to_le_bytes());
    b.extend(entry);
    b.extend(reg(REGISTER, 0x00, 64, 0x9000));
    b.extend(reg(ABSENT, 0, 0, 0));
    b
}

/// An Arm core: two own states by PSCI (fixed hardware), its cluster with
/// two, and the system with one, each above the processor's own entered
/// by an integer.
pub fn arm_core(uid: u64) -> Vec<u8> {
    let ffh = |v: u64| reg(REGISTER, 0x7F, 32, v);
    let int = |v: u64| reg(INTEGER, 0, 0, v);
    [
        levels_header(0, 0, 0, 3, uid, 0),
        level(LPI_OWN, 0, 0, 2),
        state(1, 0, ffh(0xFFFF_FFFF)),
        state(100, 2, ffh(0x4000_0003)),
        level(LPI_CONTAINER, 0x10, 1, 2),
        state(5, 0, int(0x10000)),
        state(300, 1, int(0x20000)),
        level(LPI_CONTAINER, 0x100, 2, 1),
        state(1000, 0, int(0x100_0000)),
    ]
    .concat()
}
