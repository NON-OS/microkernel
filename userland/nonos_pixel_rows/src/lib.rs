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

//! Pixel rows for the compositor and the Wayland bridge. A row is a byte
//! slice of little-endian 32-bit pixels, alpha in the top byte; it need not
//! be aligned. Each operation picks its path once per row from `level`.

#![no_std]

extern crate alloc;

pub mod blend;
pub mod composite;
pub mod level;
pub mod nearest;
pub mod opaque;
pub mod scalar;
pub mod words;

#[cfg(target_arch = "x86_64")]
pub mod avx2;
#[cfg(target_arch = "x86_64")]
pub mod sse2;

pub use blend::{flatten, over_colour, over_row, under_row};
pub use composite::composite_row;
pub use level::{level, Level};
pub use nearest::{nearest_row, NearestCols};
pub use opaque::opaque_row;
