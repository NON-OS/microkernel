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

//! A root port's PORTSC, at the operational base plus 0x44, four bytes a
//! port. Its change bits clear when written as one.

pub const PORTSC: u32 = 0x44;
pub const PORTS_MAX: u8 = 15;

pub const PORT_CCS: u32 = 1 << 0;
pub const PORT_CSC: u32 = 1 << 1;
pub const PORT_PE: u32 = 1 << 2;
pub const PORT_PEC: u32 = 1 << 3;
pub const PORT_OCC: u32 = 1 << 5;
pub const PORT_PR: u32 = 1 << 8;
/// Line status (D+, D-): K-state (01) on a connected port is a low-speed
/// device, which belongs to a companion without a reset.
pub const PORT_LS_SHIFT: u32 = 10;
pub const PORT_LS_MASK: u32 = 3 << 10;
pub const LS_K: u32 = 1;
pub const PORT_PP: u32 = 1 << 12;
/// Port owner: set, the port is the companion controller's.
pub const PORT_OWNER: u32 = 1 << 13;
/// The change bits, kept out of any write meant for others.
pub const PORT_WC: u32 = PORT_CSC | PORT_PEC | PORT_OCC;
