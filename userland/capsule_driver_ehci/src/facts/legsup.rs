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

//! The USB legacy support capability, in PCI config space at the offset
//! HCCPARAMS names (EECP): USBLEGSUP (id 01h, the BIOS- and OS-owned
//! semaphores) and USBLEGCTLSTS (the SMI enables and their status).

pub const LEGSUP_ID: u32 = 0x01;
pub const BIOS_OWNED: u32 = 1 << 16;
pub const OS_OWNED: u32 = 1 << 24;
/// USBLEGCTLSTS: SMI enables in its low word, their status (each cleared
/// by a one) in 31:29.
pub const CTLSTS_ENABLES: u16 = 0xE03F;
pub const CTLSTS_STATUS_HI: u16 = 0xE000;
