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

//! Whether a card sits in the SD slot (sd_get_cd_int), and whether its
//! write-protect switch is set (sdmmc_get_ro), straight from BIPR. A
//! reader that reads all ones has no card, and no switch to read: that is
//! the reader gone, an I/O error, not a write-protected card.

use crate::error::{Result, RtsxError};
use crate::regs::host::{BIPR, SD_EXIST, SD_WRITE_PROTECT};
use crate::setup::Driver;

pub fn present(drv: &Driver) -> bool {
    let bipr = drv.regs.r32(BIPR);
    bipr != u32::MAX && bipr & SD_EXIST != 0
}

pub fn write_protected(drv: &Driver) -> Result<bool> {
    let bipr = drv.regs.r32(BIPR);
    if bipr == u32::MAX {
        return Err(RtsxError::Gone);
    }
    Ok(bipr & SD_WRITE_PROTECT != 0)
}
