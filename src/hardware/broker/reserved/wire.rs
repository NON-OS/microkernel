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

//! The reservations as the ACPI capsule sends them: 24 bytes each, the
//! space (1 I/O, 2 memory), seven zero bytes, the base and the length,
//! little-endian. A record that names no bytes, wraps, or runs past port
//! 0xFFFF refuses the whole list.

use alloc::vec::Vec;

pub const RECORD_LEN: usize = 24;
pub const MAX_RANGES: usize = 256;
pub const IO: u8 = 1;
pub const MEMORY: u8 = 2;
const PORTS: u64 = 0x1_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reservation {
    pub space: u8,
    pub base: u64,
    pub len: u64,
}

impl Reservation {
    /// One past the last byte or port; checked not to wrap when decoded.
    pub fn end(&self) -> u64 {
        self.base.saturating_add(self.len)
    }

    fn valid(&self) -> bool {
        let Some(end) = self.base.checked_add(self.len) else { return false };
        let space = self.space == MEMORY || (self.space == IO && end <= PORTS);
        space && self.len != 0
    }
}

pub fn encode(ranges: &[Reservation]) -> Option<Vec<u8>> {
    let mut b = Vec::with_capacity(ranges.len() * RECORD_LEN);
    for r in ranges {
        b.extend_from_slice(&[r.space, 0, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&r.base.to_le_bytes());
        b.extend_from_slice(&r.len.to_le_bytes());
    }
    decode(&b).filter(|d| d == ranges).map(|_| b)
}

pub fn decode(b: &[u8]) -> Option<Vec<Reservation>> {
    if !b.len().is_multiple_of(RECORD_LEN) || b.len() / RECORD_LEN > MAX_RANGES {
        return None;
    }
    let one = |r: &[u8]| {
        let word = |at: usize| u64::from_le_bytes(r[at..at + 8].try_into().unwrap_or([0; 8]));
        let res = Reservation { space: r[0], base: word(8), len: word(16) };
        (r[1..8] == [0; 7] && res.valid()).then_some(res)
    };
    b.chunks_exact(RECORD_LEN).map(one).collect()
}
