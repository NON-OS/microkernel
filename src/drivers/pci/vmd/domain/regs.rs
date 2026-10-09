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

//! Type 0 and type 1 config header registers the assignment writes (PCI
//! Local Bus 3.0, 6.1; PCI-to-PCI Bridge 1.2, 3.2).

pub(super) const CMD_IO: u32 = 1 << 0;
pub(super) const CMD_MEM: u32 = 1 << 1;
pub(super) const CMD_MASTER: u32 = 1 << 2;

pub(super) const REG_ID: u16 = 0x00;
pub(super) const REG_COMMAND: u16 = 0x04;
pub(super) const REG_HEADER: u16 = 0x0C;
pub(super) const REG_BAR0: u16 = 0x10;
pub(super) const REG_BUSES: u16 = 0x18;
pub(super) const REG_IO_WINDOW: u16 = 0x1C;
pub(super) const REG_MEM_WINDOW: u16 = 0x20;
pub(super) const REG_PREF_WINDOW: u16 = 0x24;
pub(super) const REG_PREF_BASE_HI: u16 = 0x28;
pub(super) const REG_PREF_LIMIT_HI: u16 = 0x2C;
pub(super) const REG_IO_HI: u16 = 0x30;

/// A base above its limit: the window forwards nothing.
pub(super) const CLOSED_WINDOW: u32 = 0x0000_FFF0;
pub(super) const CLOSED_IO: u32 = 0x0000_00F0;
