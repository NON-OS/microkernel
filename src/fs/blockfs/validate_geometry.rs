// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::FIRST_ALLOC_LBA;
use super::BlockFsError;

pub(crate) fn validate_geometry() -> Result<u64, BlockFsError> {
    let geometry = crate::hardware::block_device::geometry().map_err(BlockFsError::BlockDevice)?;
    if geometry.sector_size != crate::fs::cryptoblock::SECTOR_BYTES as u32 {
        return Err(BlockFsError::InvalidGeometry);
    }
    /*
     * The volume is the window the cryptoblock layer was given, not the
     * whole device: blockfs addresses sectors inside it.
     */
    let sectors = crate::fs::cryptoblock::window_sectors().ok_or(BlockFsError::InvalidGeometry)?;
    if sectors <= FIRST_ALLOC_LBA + 1 {
        return Err(BlockFsError::InvalidGeometry);
    }
    Ok(sectors)
}
