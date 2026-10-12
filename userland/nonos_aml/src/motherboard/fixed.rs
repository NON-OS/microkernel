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

//! A template's ranges, every descriptor that names I/O or memory: the
//! small I/O Port and Fixed Location I/O Port descriptors (6.4.2.5,
//! 6.4.2.6), the large 24-bit, 32-bit and 32-bit Fixed Memory Range
//! descriptors (6.4.3.1, 6.4.3.3, 6.4.3.4), and the address space ones
//! (`space`). A relocatable range keeps the whole span it may sit in, from
//! its minimum to its maximum base plus its length: firmware keeps all of
//! it, wherever the device was put.

use super::space::address;
use crate::platform::{items, Item};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Space {
    Io,
    Memory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reserved {
    pub space: Space,
    pub base: u64,
    pub len: u64,
}

pub(super) fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |a, x| a << 8 | u64::from(*x))
}

/// Minimum, maximum base and length: the span [min, max + len).
fn span(space: Space, min: u64, max: u64, len: u64) -> Option<Reserved> {
    let end = max.max(min).checked_add(len)?;
    (len != 0).then_some(Reserved { space, base: min, len: end - min })
}

fn one(i: &Item<'_>) -> Option<Reserved> {
    let b = i.body;
    let (io, mem) = (Space::Io, Space::Memory);
    match (i.large, i.kind) {
        (false, 0x08) if b.len() >= 7 => span(io, le(&b[1..3]), le(&b[3..5]), u64::from(b[6])),
        // ten address bits, as the ISA bus decoded them
        (false, 0x09) if b.len() >= 3 => span(io, le(&b[0..2]) & 0x3FF, 0, u64::from(b[2])),
        (true, 0x01) if b.len() >= 9 => {
            span(mem, le(&b[1..3]) << 8, le(&b[3..5]) << 8, le(&b[7..9]) << 8)
        }
        (true, 0x05) if b.len() >= 17 => span(mem, le(&b[1..5]), le(&b[5..9]), le(&b[13..17])),
        (true, 0x06) if b.len() >= 9 => span(mem, le(&b[1..5]), 0, le(&b[5..9])),
        (true, _) => address(i),
        _ => None,
    }
}

/// In template order. An I/O range past port 0xFFFF is left out, as is
/// any range that would wrap the address space.
pub fn reserved(t: &[u8]) -> Vec<Reserved> {
    let fits = |r: &Reserved| r.space == Space::Memory || r.base + r.len <= 0x1_0000;
    items(t).iter().filter_map(one).filter(fits).collect()
}
