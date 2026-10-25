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

use super::super::dte::{stray_writer, Dte};
use super::super::error::AmdViError;
use super::super::regs::DEV_TABLE_PAGES;
use super::table::PER_PAGE;
use crate::arch::x86_64::iommu::tables::view::Frame;

/// Refuse before a unit is enabled if any entry would let a device write
/// untranslated memory. The kernel writes only blocked entries or translated
/// page tables, so a stray writer means the table was left tampered with or
/// half-written; enabling over it would defeat the unit's whole purpose.
pub(in super::super) fn deny_stray_writers(table: u64) -> Result<(), AmdViError> {
    for page in 0..DEV_TABLE_PAGES {
        let words =
            Frame::at(table + page as u64 * 4096).map_err(|_| AmdViError::TableUnreachable)?;
        for at in (0..PER_PAGE * 4).step_by(4) {
            let dte: Dte =
                [words.read(at)?, words.read(at + 1)?, words.read(at + 2)?, words.read(at + 3)?];
            if stray_writer(dte) {
                return Err(AmdViError::StrayWriter);
            }
        }
    }
    Ok(())
}
