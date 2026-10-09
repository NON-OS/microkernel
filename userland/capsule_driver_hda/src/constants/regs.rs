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

//! Controller registers, by the offsets of the Intel High Definition Audio
//! Specification 1.0a, section 3.3.

pub const GCAP: u32 = 0x00;
pub const VMIN: u32 = 0x02;
pub const VMAJ: u32 = 0x03;
pub const OUTPAY: u32 = 0x04;
pub const INPAY: u32 = 0x06;
pub const GCTL: u32 = 0x08;
pub const STATESTS: u32 = 0x0e;
pub const GSTS: u32 = 0x10;
pub const INTCTL: u32 = 0x20;
pub const INTSTS: u32 = 0x24;
/// Immediate Command interface (HDA 1.0a section 3.4.3): one verb out through
/// ICOI, its answer back through ICII, for reading a codec parameter without
/// the CORB ring.
pub const ICOI: u32 = 0x60;
pub const ICII: u32 = 0x64;
pub const ICIS: u32 = 0x68;
/// ICIS bit 0: a command is in flight (Immediate Command Busy).
pub const ICIS_ICB: u16 = 1 << 0;
/// ICIS bit 1: the answer in ICII is valid (Immediate Result Valid).
pub const ICIS_IRV: u16 = 1 << 1;

pub const DPLBASE: u32 = 0x70;
pub const DPUBASE: u32 = 0x74;

pub const GCTL_CRST: u32 = 1 << 0;
/// The fifteen codec slots STATESTS reports; bit 15 is reserved.
pub const STATESTS_MASK: u16 = 0x7fff;
/// DPLBASE bit 0: the controller writes each stream's position to memory.
pub const DPLBASE_ENABLE: u32 = 1 << 0;

pub const INTCTL_GIE: u32 = 1 << 31;

/// Intel vendor-specific register at 0x1040 (Linux `AZX_REG_VS_EM4L`).
pub const VS_EM4L: u32 = 0x1040;

pub const STREAM_TAG: u8 = 1;

/// GET_PARAMETER and the vendor id parameter, the first verb every codec
/// is asked; the rest of the codec verbs are in `verbs`.
pub const VERB_GET_PARAMETER: u16 = 0xf00;
pub const PARAM_VENDOR_ID: u16 = 0x00;
