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

use crate::fs::cryptoblock::CryptoBlockError;
use crate::hardware::block_device::BlockDeviceError;

use super::tree_store::TreeFault;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockFsError {
    BlockDevice(BlockDeviceError),
    CryptoBlock(CryptoBlockError),
    InvalidGeometry,
    InvalidSuperblock,
    InvalidRecord,
    InvalidName,
    DirectoryFull,
    OutOfSpace,
    NotFound,
    NotFormatted,
}

/// A tree fault in the filesystem's terms.
pub(super) fn fault(f: TreeFault<BlockFsError>) -> BlockFsError {
    match f {
        TreeFault::Store(e) => e,
        TreeFault::TooLarge => BlockFsError::OutOfSpace,
        TreeFault::Hole => BlockFsError::InvalidRecord,
    }
}
