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

//! Where Intel pads are: the communities, each a `_CRS` window with its
//! pad stride and its groups' interrupt status and enable bases, and the
//! caller's placement of a firmware pin among them.

use crate::window::Window;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntelPad {
    pub community: usize,
    pub pad: u16,
    pub group: u8,
    pub bit: u8,
}

pub struct Community<W: Window> {
    pub window: W,
    pub stride: u32,
    pub gpi_is: u32,
    pub gpi_ie: u32,
}

pub struct IntelGpio<W: Window> {
    pub communities: Vec<Community<W>>,
    pub place: fn(u16) -> Option<IntelPad>,
}
