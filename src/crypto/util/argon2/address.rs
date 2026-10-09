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

//! The address blocks of data-independent addressing, RFC 9106 section
//! 3.4.1.2: each is G(0, G(0, Z)), Z naming the segment and a counter.

use super::block::{compress, Block};
use super::index::{Position, Shape};
use super::wipe::wipe;

pub(super) struct Addresses {
    input: Block,
    out: Block,
}

impl Addresses {
    pub(super) fn new(shape: &Shape, at: Position) -> Self {
        let mut input = [0u64; 128];
        input[0] = at.pass as u64;
        input[1] = at.lane as u64;
        input[2] = at.slice as u64;
        input[3] = shape.blocks as u64;
        input[4] = shape.passes as u64;
        input[5] = shape.y as u64;
        Self { input, out: [0u64; 128] }
    }

    /// Advance the counter and make the next 128 pseudo-random words.
    pub(super) fn next(&mut self) {
        let zero = [0u64; 128];
        self.input[6] += 1;
        let mut once = [0u64; 128];
        compress(&zero, &self.input, &mut once, false);
        compress(&zero, &once, &mut self.out, false);
        wipe(&mut once);
    }

    pub(super) fn word(&self, i: usize) -> u64 {
        self.out[i % 128]
    }
}
