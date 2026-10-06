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

//! The user area size, from EXT_CSD or from the CSD.

use super::super::ext_csd::ExtCsd;
use super::super::regs::Csd;

/// The user area in sectors: SEC_COUNT on a sector-mode card, the CSD
/// computation on a byte-mode one; 0 when neither gives one.
pub fn capacity(sector_mode: bool, csd: &Csd, ext: Option<&ExtCsd>) -> u64 {
    if sector_mode {
        ext.map_or(0, |e| e.sec_count() as u64)
    } else {
        csd.sectors().unwrap_or(0)
    }
}
