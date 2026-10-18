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

//! The compositor's products of sizes held at u32's edges, each
//! against the test's own reading in u128: the canvas and its backing, the
//! lens, the pointer's place, the fitted box and its samples, the extended
//! layout, and damage.

mod blit_edges;
mod canvas_edges;
#[path = "../../../../compositor/src/setup/canvas_floor.rs"]
pub mod canvas_floor;
#[path = "../../../../compositor/src/setup/canvas_scale.rs"]
mod canvas_scale;
mod damage_edges;
#[path = "../../../../gfx_serve_proofs/src/sweep/edges.rs"]
mod edges;
mod layout_edges;
mod lens_edges;

/// A seeded walk over the edges.
pub struct Pick(pub u64);

impl Pick {
    pub fn edge(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        edges::EDGES[(self.0 % edges::EDGES.len() as u64) as usize]
    }
}
