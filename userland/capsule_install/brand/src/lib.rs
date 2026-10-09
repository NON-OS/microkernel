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

//! NØNOS's look for the installer and first-boot setup: its faces, palette,
//! mark and frame, the same as the bootloader's screens.

#![no_std]

extern crate alloc;

mod faces;
mod frame;
mod labels;
mod mark;
pub mod palette;
mod pixels;
mod release;
pub mod scale;
mod type_set;

pub use faces::Face;
pub use frame::emblem;
pub use labels::{label, label_w, marker};
pub use mark::mark;
pub use release::release;
pub use type_set::{line_h, measure, text};
