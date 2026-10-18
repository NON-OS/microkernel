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

//! An Intel PCH GPIO community (Sunrise Point and later). Recalled from
//! the PCH datasheets' GPIO register chapters (volume 2), which could not
//! be read here: unverified. The register at 0x0C holds the offset of the
//! first pad's configuration; each pad then takes a stride (8 bytes on the
//! older parts, 16 on the newer); each pad group of up to 32 pads has one
//! interrupt status dword (write one to clear) and one enable dword, at
//! bases that differ by generation and so come from the caller.

pub const PADBAR: u32 = 0x0C;

/// PAD_CFG_DW0.
pub const TX_STATE: u32 = 1 << 0;
pub const RX_STATE: u32 = 1 << 1;
pub const TX_DISABLE: u32 = 1 << 8;
pub const RX_DISABLE: u32 = 1 << 9;
/// Three bits; 0 is GPIO, anything else a native function.
pub const PMODE_MASK: u32 = 7 << 10;
pub const RX_INVERT: u32 = 1 << 23;
/// Two bits: 0 level, 1 edge, 2 driven low, 3 both edges.
pub const RX_EVCFG_SHIFT: u32 = 25;
pub const RX_EVCFG_MASK: u32 = 3 << RX_EVCFG_SHIFT;
