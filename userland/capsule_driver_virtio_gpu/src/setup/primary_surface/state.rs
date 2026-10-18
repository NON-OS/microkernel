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
/// The primary surface: what the compositor is handed, and the frames
/// behind it, which a recovery attaches again to the resource it remakes.
#[derive(Clone, Copy)]
pub struct Primary {
    pub handle: u64,
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Where the device finds the surface's frames, and how many bytes.
    pub backing_addr: u64,
    pub backing_len: u32,
}
