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

//! CORB and RIRB registers and their bits, by the Intel High Definition
//! Audio Specification 1.0a, section 3.3.

pub const CORBLBASE: u32 = 0x40;
pub const CORBUBASE: u32 = 0x44;
pub const CORBWP: u32 = 0x48;
pub const CORBRP: u32 = 0x4a;
pub const CORBCTL: u32 = 0x4c;
pub const CORBSIZE: u32 = 0x4e;
pub const RIRBLBASE: u32 = 0x50;
pub const RIRBUBASE: u32 = 0x54;
pub const RIRBWP: u32 = 0x58;
pub const RINTCNT: u32 = 0x5a;
pub const RIRBCTL: u32 = 0x5c;
pub const RIRBSTS: u32 = 0x5d;
pub const RIRBSIZE: u32 = 0x5e;

pub const CORBCTL_RUN: u8 = 1 << 1;
pub const CORBRP_RST: u16 = 1 << 15;
/// CORBSIZE and RIRBSIZE share one layout: the size code in bits 1:0 and
/// the sizes the controller supports in bits 7:4 (2, 16 and 256 entries).
pub const RINGSIZE_2: u8 = 0x00;
pub const RINGSIZE_16: u8 = 0x01;
pub const RINGSIZE_256: u8 = 0x02;
pub const RINGSIZE_CAP_2: u8 = 1 << 4;
pub const RINGSIZE_CAP_16: u8 = 1 << 5;
pub const RINGSIZE_CAP_256: u8 = 1 << 6;
pub const RIRBCTL_DMAEN: u8 = 1 << 1;
pub const RIRBCTL_RINTCTL: u8 = 1 << 0;
pub const RIRBWP_RST: u16 = 1 << 15;
pub const RIRBSTS_INTFL: u8 = 1 << 0;
pub const RINTCNT_ONE: u16 = 1;
/// The upper word of a RIRB entry: the answering codec's address in bits 3:0
/// and, in bit 4, whether the entry is an unsolicited response.
pub const RIRB_EX_CODEC: u32 = 0x0f;
pub const RIRB_EX_UNSOL: u32 = 1 << 4;
