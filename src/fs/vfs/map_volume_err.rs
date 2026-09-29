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

use super::error::VfsError;
use crate::fs::blockfs::BlockFsError;
use crate::fs::blockfs_volume::VolumeError;

pub(super) fn map_volume_err(e: VolumeError) -> VfsError {
    match e {
        VolumeError::NotMounted => VfsError::NotInitialized,
        VolumeError::BadKeyLength => VfsError::IoError("blockfs key length"),
        VolumeError::Keyring(_) => VfsError::IoError("keyring unavailable"),
        VolumeError::TooLargeToReadWhole(_) => {
            VfsError::FsError("file too large to read whole; read it by range")
        }
        VolumeError::Plan(_) => VfsError::IoError("no usable disk plan"),
        VolumeError::MachineKey(_) => VfsError::IoError("no machine key for the data volume"),
        VolumeError::Device(_) => VfsError::IoError("block device"),
        VolumeError::Window(_) => VfsError::IoError("data volume window"),
        VolumeError::NoImport => VfsError::NotFound,
        VolumeError::DigestMismatch => VfsError::IoError("import does not match its pinned digest"),
        VolumeError::NameTaken => VfsError::AlreadyExists,
        VolumeError::Unopenable => VfsError::IoError("data volume under another key"),
        VolumeError::BlockFs(BlockFsError::NotFound) => VfsError::NotFound,
        VolumeError::BlockFs(_) => VfsError::IoError("blockfs"),
    }
}
