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

//! The live state the server loop holds.

use crate::fifo::Fifo;
use crate::regs::Regs;
use crate::screen::Plane;

pub struct Driver {
    pub regs: Regs,
    pub fifo: Fifo,
    /// The visible framebuffer in BAR1, at the device's line pitch.
    pub fb: Plane,
    /// The compositor's surface, and the handle it attaches with.
    pub surface: Plane,
    pub handle: u64,
    /// The first pid to ask for the primary surface; only it may draw.
    pub owner: u32,
}
