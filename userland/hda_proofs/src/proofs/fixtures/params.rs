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

//! The parameter ids the sim answers, and the capability words the
//! described widgets report.

pub const P_VENDOR: u16 = 0x00;
pub const P_SUBNODES: u16 = 0x04;
pub const P_FG_TYPE: u16 = 0x05;
pub const P_WCAPS: u16 = 0x09;
pub const P_PCM: u16 = 0x0a;
pub const P_PINCAP: u16 = 0x0c;
pub const P_AMP_IN: u16 = 0x0d;
pub const P_CONNLEN: u16 = 0x0e;
pub const P_GPIO: u16 = 0x11;
pub const P_AMP_OUT: u16 = 0x12;

pub const PCM_48K16: u32 = 0x000e_0560;
pub const DAC_WCAPS: u32 = 0x0000_041d;
pub const DAC_AMP: u32 = 0x0005_7057;
pub const PIN_OUT_WCAPS: u32 = 0x0040_058d;
pub const PIN_IN_WCAPS: u32 = 0x0040_048b;
pub const NC: u32 = 0x4111_11f0;
pub const HP_SUBSYSTEM: u32 = 0x103c_8651;
