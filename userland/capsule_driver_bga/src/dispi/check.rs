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

//! What the adapter says it is: the DISPI interface version, read before
//! anything else of it is believed. The mode it holds is read back by
//! `current`.

use super::dispi_off::dispi_off;
use crate::constants::{DISPI_ID_FIRST, DISPI_ID_LAST, DISPI_INDEX_ID};
use crate::error::{BgaError, BgaResult};
use crate::regs::Regs;

/// Whether the ID register names a DISPI version this driver programs.
/// Anything else behind this PCI ID, or a window that reads all ones, is
/// not driven.
pub fn identify(regs: Regs) -> BgaResult<()> {
    // SAFETY: the ID register lies below `regs_len`, which the caller has
    // checked the register mapping covers.
    let id = unsafe { regs.r16(dispi_off(DISPI_INDEX_ID)) };
    if (DISPI_ID_FIRST..=DISPI_ID_LAST).contains(&id) {
        Ok(())
    } else {
        Err(BgaError::NotDispi)
    }
}
