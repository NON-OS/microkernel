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
//! HCCPARAMS1 MaxPSASize, bits 15:12: the largest Primary Stream Array the
//! controller takes is 2^(MaxPSASize + 1) entries; 0 means it does not do
//! streams (xHCI 1.2, 5.3.6, as recalled, unverified).

use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

pub fn max_psa(mmio_base: u64) -> u8 {
    ((mmio_read32(mmio_base + HCCPARAMS1) >> 12) & 0xF) as u8
}
