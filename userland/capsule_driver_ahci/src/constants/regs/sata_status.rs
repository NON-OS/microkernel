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

//! The SATA status and control fields: DET and the interface power state.

// SATA status/control DET field (PxSSTS / PxSCTL bits [3:0]).
pub const SSTS_DET_MASK: u32 = 0xf;
pub const SSTS_DET_PRESENT: u32 = 0x3; // device present, PHY communication up
pub const SSTS_DET_DETECTING: u32 = 0x1; // device seen, no communication yet
pub const SSTS_IPM_SHIFT: u32 = 8;
pub const SSTS_IPM_MASK: u32 = 0xf;
pub const IPM_ACTIVE: u32 = 0x1;
pub const IPM_PARTIAL: u32 = 0x2;
pub const IPM_SLUMBER: u32 = 0x6;
pub const IPM_DEVSLEEP: u32 = 0x8;
pub const SCTL_DET_MASK: u32 = 0xf;
pub const SCTL_DET_COMRESET: u32 = 0x1;
