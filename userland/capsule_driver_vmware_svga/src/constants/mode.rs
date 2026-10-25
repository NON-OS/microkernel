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

/// A mode the firmware did not set: the classic VMware default.
pub const DEFAULT_MODE: (u32, u32) = (1024, 768);
/// Smaller modes, tried in order when VRAM cannot hold the default.
pub const FALLBACK_MODES: [(u32, u32); 2] = [(800, 600), (640, 480)];
pub const BITS_PER_PIXEL: u32 = 32;
pub const BYTES_PER_PIXEL: u32 = 4;
