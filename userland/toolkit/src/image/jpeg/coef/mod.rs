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

//! Coefficient-buffered JPEG decoding: baseline, extended and progressive
//! frames, DQT/DHT between scans, restart intervals, and an IDCT that
//! reads 8, 4, 2 or 1 coefficients a side to decode at 1/1..1/8 scale.

mod ac_correct;
mod ac_first;
mod ac_refine;
mod blocks;
mod color;
mod cost;
mod dc_scans;
mod decode;
mod emit;
mod frame;
mod huff;
mod idct;
mod marker;
mod plane;
mod refuse;
mod samples;
mod scan;
mod state;
mod upsample;
mod walk;
mod walk_block;

pub use cost::{cost, Cost};
pub use decode::decode_scaled;
