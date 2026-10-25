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

//! The capability bits CAPS carries in its flags word.

pub const MODE_SET: u32 = 1 << 0;
/// Any size from 640x480 to the maximum, not only the listed modes.
pub const MODE_RANGE: u32 = 1 << 1;
pub const HW_CURSOR: u32 = 1 << 2;
/// STATE reports the host's size hints.
pub const HINTS: u32 = 1 << 3;
/// The device reads guest memory: the primary is a DMA grant.
pub const DEVICE_READS: u32 = 1 << 4;
