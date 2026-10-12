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

//! The engine as a display-wire device (`nonos_gfx_serve::Display`): one
//! taken-over output, its two scanout surfaces filled from the shadow by
//! FLUSH and flipped at the vertical blank.

mod copy;
mod describe;
mod frames;
mod health;
mod intel;
mod monitor;
mod panel_fits;
mod red_first;
mod wire;

pub use copy::copy;
pub use frames::Frames;
pub use intel::{Intel, FLIP_US};
pub use monitor::{panel, panel_of, read as read_edid};
pub use panel_fits::fits;
pub use red_first::{red_first, swap_red_blue};
