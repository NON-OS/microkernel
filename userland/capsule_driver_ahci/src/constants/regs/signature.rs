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

//! The device signatures a port reports in PxSIG.

pub const SIG_SATA: u32 = 0x0000_0101;
pub const SIG_ATAPI: u32 = 0xeb14_0101;
pub const SIG_SEMB: u32 = 0xc33c_0101;
pub const SIG_PM: u32 = 0x9669_0101;
