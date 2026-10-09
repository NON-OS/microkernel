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

//! Bits of the port command, task file, interrupt and control registers.

pub const CMD_ST: u32 = 1 << 0;
pub const CMD_FRE: u32 = 1 << 4;
pub const CMD_FR: u32 = 1 << 14;
pub const CMD_CR: u32 = 1 << 15;
pub const CMD_POD: u32 = 1 << 2;
pub const CMD_SUD: u32 = 1 << 1;
/// PxCMD.CLO: clear BSY and DRQ in PxTFD so a stuck port can take a command.
pub const CMD_CLO: u32 = 1 << 3;

pub const TFD_ERR: u32 = 1 << 0;
pub const TFD_DRQ: u32 = 1 << 3;
pub const TFD_BSY: u32 = 1 << 7;
pub const IS_ERR_MASK: u32 = 1 << 30 | 1 << 29 | 1 << 28 | 1 << 27;
/// PxIS.OFS: the device sent more data than the PRD entries describe.
pub const IS_OFS: u32 = 1 << 24;
pub const SCTL_IPM_DISABLE: u32 = 7 << 8;
pub const PORT_IE_DEFAULT: u32 = 0b1_0111;
