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
//! The driver's own blank surface: a resource the size of the
//! primary whose backing is DMA the kernel zeroed when it was mapped and
//! that no other process maps. It goes to the host whole once at bring-up
//! and is never written again, so it is what a dead owner's screen shows
//! in place of its last frame.

#[derive(Clone, Copy)]
pub struct Blank {
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
}
