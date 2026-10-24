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

use super::super::config::ConfigSpace;
use super::super::constants::*;
use super::super::error::Result;
use super::super::types::MsixInfo;

pub fn enable_msix(config: &ConfigSpace, msix: &MsixInfo) -> Result<()> {
    let offset = msix.offset as u16;
    let mut ctrl = config.read16(offset + 2)?;
    ctrl |= MSIX_CTRL_ENABLE;
    ctrl &= !MSIX_CTRL_FUNCTION_MASK;
    config.write16(offset + 2, ctrl)?;
    Ok(())
}

pub fn disable_msix(config: &ConfigSpace, msix: &MsixInfo) -> Result<()> {
    let offset = msix.offset as u16;
    let mut ctrl = config.read16(offset + 2)?;
    ctrl &= !MSIX_CTRL_ENABLE;
    config.write16(offset + 2, ctrl)?;
    Ok(())
}

pub fn is_msix_enabled(config: &ConfigSpace, msix: &MsixInfo) -> Result<bool> {
    let offset = msix.offset as u16;
    let ctrl = config.read16(offset + 2)?;
    Ok((ctrl & MSIX_CTRL_ENABLE) != 0)
}
