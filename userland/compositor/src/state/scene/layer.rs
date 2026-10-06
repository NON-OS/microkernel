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

pub const MAX_LAYERS: usize = 32;

/// The most layers one process may hold, one per z band. An application holds
/// one; the desktop shell holds two (its desktop under the windows and its
/// chrome over them). The cap keeps one client from filling the table.
pub const MAX_LAYERS_PER_OWNER: usize = 4;

#[derive(Clone, Copy, Default)]
pub struct Layer {
    pub owner_pid: u32,
    pub surface_handle: u64,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub z: u32,
    /// Raise order inside the layer's z band: the layer raised (or first
    /// submitted) most recently has the highest value and draws over every
    /// other layer of its band. The window manager raises a window on every
    /// click, so this mirrors its stack and the window drawn on top is the
    /// window its hit test gives the next click to.
    pub stack: u32,
    pub in_use: bool,
    pub miss_count: u16,
}
