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

//! Whether a volume's header ring has never been written.

use super::error::VolumeError;
use crate::fs::blockfs::HEADER_RING_SECTORS;

/// True when every sector of the header ring at device LBA `base` is zero.
pub(super) fn ring_blank(base: u64) -> Result<bool, VolumeError> {
    let mut sector = [0u8; 512];
    for lba in base..base + HEADER_RING_SECTORS {
        crate::hardware::block_device::read(lba, &mut sector).map_err(VolumeError::Device)?;
        if sector.iter().any(|b| *b != 0) {
            return Ok(false);
        }
    }
    Ok(true)
}
