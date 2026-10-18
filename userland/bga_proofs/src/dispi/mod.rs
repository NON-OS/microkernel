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

//! The DISPI block and the framebuffer, from the shipping files. The
//! capsule keeps the index arithmetic sealed in its own tree; named here as
//! siblings, it is reachable to the tests through the same path the mode
//! set uses.

#[path = "../../../capsule_driver_bga/src/dispi/check.rs"]
mod check;
#[path = "../../../capsule_driver_bga/src/dispi/clear.rs"]
mod clear;
#[path = "../../../capsule_driver_bga/src/dispi/copy.rs"]
mod copy;
#[path = "../../../capsule_driver_bga/src/dispi/dispi_off.rs"]
mod dispi_off;
#[path = "../../../capsule_driver_bga/src/dispi/fit.rs"]
mod fit;
#[path = "../../../capsule_driver_bga/src/dispi/restore.rs"]
mod restore;
#[path = "../../../capsule_driver_bga/src/dispi/set_mode.rs"]
mod set_mode;
#[path = "../../../capsule_driver_bga/src/dispi/span.rs"]
mod span;
#[path = "../../../capsule_driver_bga/src/dispi/state.rs"]
mod state;

pub use check::{confirm, identify, kept};
pub use clear::clear;
pub use copy::{copy, place, Rect};
pub use dispi_off::dispi_off;
pub use fit::{fit, video_memory};
pub use restore::{restore, write_back};
pub use set_mode::{regs_len, set_mode};
pub use span::{span, Span};
pub use state::{read, Dispi};
