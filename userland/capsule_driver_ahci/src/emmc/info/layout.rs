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

//! The reply sizes and the fixed values the payloads carry.

/// IDENTIFY's medium byte for an eMMC part.
pub const MEDIUM_EMMC: u8 = 1;
pub const MODEL_MAX: usize = 40;
pub const SERIAL_LEN: usize = 8;

pub const CONTROLLER_INFO_LEN: usize = 24;
pub const PORT_ENTRY_LEN: usize = 36;
pub const PORT_KIND_EMMC: u8 = 5;
