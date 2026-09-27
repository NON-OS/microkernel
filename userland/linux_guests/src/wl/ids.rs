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

//! The object ids this client allocates, in the order it creates them.
//! wl_display is 1 by the protocol.

pub const REG: u32 = 2;
pub const SYNC: u32 = 3;
pub const COMP: u32 = 4;
pub const SHM: u32 = 5;
pub const XDG: u32 = 6;
pub const SURF: u32 = 7;
pub const XSURF: u32 = 8;
pub const TOP: u32 = 9;
pub const POOL: u32 = 10;
pub const BUF: u32 = 11;
