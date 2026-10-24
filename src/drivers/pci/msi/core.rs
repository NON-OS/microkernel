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
use super::super::error::{PciError, Result};
use super::super::types::MsiInfo;

pub fn disable_msi(config: &ConfigSpace, msi: &MsiInfo) -> Result<()> {
    let offset = msi.offset as u16;
    let mut ctrl = config.read16(offset + 2)?;
    ctrl &= !MSI_CTRL_ENABLE;
    config.write16(offset + 2, ctrl)?;
    Ok(())
}

pub fn is_msi_enabled(config: &ConfigSpace, msi: &MsiInfo) -> Result<bool> {
    let offset = msi.offset as u16;
    let ctrl = config.read16(offset + 2)?;
    Ok((ctrl & MSI_CTRL_ENABLE) != 0)
}

pub fn mask_msi_vector(config: &ConfigSpace, msi: &MsiInfo, vector: u8) -> Result<()> {
    if !msi.per_vector_mask {
        return Err(PciError::MsiNotSupported);
    }

    let mask_offset = if msi.is_64bit { msi.offset as u16 + 16 } else { msi.offset as u16 + 12 };

    let mut mask = config.read32(mask_offset)?;
    mask |= 1u32 << vector;
    config.write32(mask_offset, mask)?;

    Ok(())
}

pub fn unmask_msi_vector(config: &ConfigSpace, msi: &MsiInfo, vector: u8) -> Result<()> {
    if !msi.per_vector_mask {
        return Err(PciError::MsiNotSupported);
    }

    let mask_offset = if msi.is_64bit { msi.offset as u16 + 16 } else { msi.offset as u16 + 12 };

    let mut mask = config.read32(mask_offset)?;
    mask &= !(1u32 << vector);
    config.write32(mask_offset, mask)?;

    Ok(())
}
