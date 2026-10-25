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

//! The memory the compositor draws into, as the display path attached it.

/// The memory the compositor draws into, as the path attached it.
#[derive(Clone, Copy)]
pub struct Primary {
    pub handle: u64,
    pub va: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub byte_len: u64,
}
