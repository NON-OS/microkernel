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

//! MODES (0x11): count u32 (0-20), then count modes of width, height and
//! flags, u32 each.

pub const MAX_MODES: usize = 20;
pub const MODE_LEN: usize = 12;

pub const PREFERRED: u32 = 1 << 0;
pub const CURRENT: u32 = 1 << 1;
pub const FROM_DEVICE: u32 = 1 << 2;
pub const FROM_EDID: u32 = 1 << 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub flags: u32,
}
