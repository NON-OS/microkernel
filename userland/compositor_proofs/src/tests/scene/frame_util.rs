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

//! The scene proofs' small parts: a layer as a client submits it, and a
//! deterministic generator, so a failing sequence replays exactly.

use crate::scene::Layer;

/// A layer as a client submits it: the stack stamp is the scene's to assign.
pub fn layer(owner: u32, x: u32, y: u32, w: u32, h: u32, z: u32) -> Layer {
    Layer {
        owner_pid: owner,
        surface_handle: owner as u64,
        x,
        y,
        width: w,
        height: h,
        z,
        stack: 0,
        in_use: true,
        miss_count: 0,
        cursor: None,
    }
}

/// A small deterministic generator, so a failing sequence replays exactly.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn below(&mut self, n: u32) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n.max(1) as u64) as u32
    }
}
