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

//! The HBA's global registers: capabilities, global control, and the
//! BIOS/OS handoff.

pub const HBA_CAP: u32 = 0x00;
pub const HBA_GHC: u32 = 0x04;
pub const HBA_IS: u32 = 0x08;
pub const HBA_PI: u32 = 0x0c;
pub const HBA_VS: u32 = 0x10;
pub const HBA_CAP2: u32 = 0x24;
/// BIOS/OS Handoff Control and Status.
pub const HBA_BOHC: u32 = 0x28;

/// CAP.NP: number of ports, less one.
pub const CAP_NP_MASK: u32 = 0x1f;
/// CAP.CPD: ports have cold presence detection, so PxCMD.POD is writable.
pub const CAP_CPD: u32 = 1 << 20;
/// CAP.SCLO: the HBA supports PxCMD.CLO.
pub const CAP_SCLO: u32 = 1 << 24;
/// CAP.S64A: the HBA takes 64-bit DMA addresses.
pub const CAP_S64A: u32 = 1 << 31;
/// CAP2.BOH: the HBA has BIOS/OS handoff.
pub const CAP2_BOH: u32 = 1 << 0;
pub const BOHC_BOS: u32 = 1 << 0;
pub const BOHC_OOS: u32 = 1 << 1;
pub const BOHC_BB: u32 = 1 << 4;

pub const GHC_AE: u32 = 1 << 31;
pub const GHC_HR: u32 = 1 << 0;
