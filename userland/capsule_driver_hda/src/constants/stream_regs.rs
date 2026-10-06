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

//! Stream descriptor registers and their bits, by the Intel High
//! Definition Audio Specification 1.0a, section 3.3.

pub const SD_CTL: u32 = 0x00;
pub const SD_STS: u32 = 0x03;
pub const SD_LPIB: u32 = 0x04;
pub const SD_CBL: u32 = 0x08;
pub const SD_LVI: u32 = 0x0c;
pub const SD_FMT: u32 = 0x12;
pub const SD_BDPL: u32 = 0x18;
pub const SD_BDPU: u32 = 0x1c;
/// Each stream descriptor is 0x20 bytes, the first at 0x80.
pub const SD_BASE: u32 = 0x80;
pub const SD_STRIDE: u32 = 0x20;

pub const SDCTL_SRST: u8 = 1 << 0;
pub const SDCTL_RUN: u8 = 1 << 1;
pub const SDCTL_IOCE: u8 = 1 << 2;
/// The three interrupt enables in SDnCTL bits 4:2.
pub const SDCTL_INT_MASK: u8 = 0x1c;
/// SDnCTL bit 19, in the third byte: a bidirectional engine runs as output.
pub const SDCTL2_DIR_OUT: u8 = 1 << 3;
pub const SDSTS_BCIS: u8 = 1 << 2;
/// The three latched status bits in SDnSTS 4:2, cleared by writing ones.
pub const SDSTS_MASK: u8 = 0x1c;
