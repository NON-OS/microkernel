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


//! The Intel pad layouts this crate places pins with: none. Every table it
//! carried was transcribed from another operating system's source rather
//! than from a published document, so all were dropped (F 46). A layout
//! comes back only when it is derived from Intel's published PCH datasheet
//! (the GPIO pad lists, volume 1 or 2), cited by document and section.
//! Until then an Intel controller's pins are refused, not guessed, and its
//! drivers poll.

use crate::group::Layout;

/// Each Intel GPIO `_HID` and the layout it binds to.
pub const LAYOUTS: &[(&[&[u8]], &Layout)] = &[];
