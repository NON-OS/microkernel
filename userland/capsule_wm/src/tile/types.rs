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
//! The words the layout is written in.

/// A window: its owner's pid and its id, as the window table keys it.
pub type Win = (u32, u32);

/// How a split lays its children out: side by side, or one above another.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Split {
    Across,
    Down,
}

/// A direction to move focus or a window in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

/// Pixels between tiled windows, and between them and the work area's edge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Gaps {
    pub inner: u32,
    pub outer: u32,
}
