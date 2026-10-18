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
//! The framebuffer size recorded on the last paint, so the pointer handlers
//! hit-test the same layout the screens draw.

pub struct Frame {
    /// Width, for the width-relative layout.
    pub w: u32,
    /// Height, for the height-relative layout.
    pub h: u32,
}

impl Frame {
    pub fn new() -> Self {
        Self { w: 1280, h: 0 }
    }
}
