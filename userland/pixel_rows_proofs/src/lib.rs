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

//! Host proofs for `nonos_pixel_rows`, its source included as it ships.

extern crate alloc;

#[path = "../../nonos_pixel_rows/src/blend.rs"]
pub mod blend;
#[path = "../../nonos_pixel_rows/src/composite.rs"]
pub mod composite;
#[path = "../../nonos_pixel_rows/src/level.rs"]
pub mod level;
#[path = "../../nonos_pixel_rows/src/nearest.rs"]
pub mod nearest;
#[path = "../../nonos_pixel_rows/src/opaque.rs"]
pub mod opaque;
#[path = "../../nonos_pixel_rows/src/scalar.rs"]
pub mod scalar;
#[path = "../../nonos_pixel_rows/src/words.rs"]
pub mod words;

#[cfg(target_arch = "x86_64")]
#[path = "../../nonos_pixel_rows/src/avx2.rs"]
pub mod avx2;
#[cfg(target_arch = "x86_64")]
#[path = "../../nonos_pixel_rows/src/sse2.rs"]
pub mod sse2;

/// The per-pixel routines the rows replaced, word for word.
pub mod reference;
/// A small deterministic generator, so a failing case replays exactly.
pub mod rng;

#[cfg(test)]
mod tests;
