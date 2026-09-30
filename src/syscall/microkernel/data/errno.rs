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

//! A volume refusal as the errno a caller sees.

use super::super::errnos::{ERRNO_ACCES, ERRNO_BUSY, ERRNO_EXIST, ERRNO_NOENT, ERRNO_NOMEM};
use crate::crypto::util::argon2::Argon2Error;
use crate::fs::blockfs::BlockFsError;
use crate::fs::blockfs_volume::VolumeError;
use crate::hardware::block_device::BlockDeviceError;

/// EIO: the volume or the disk under it could not do what was asked.
pub(super) const ERRNO_IO: i64 = -5;
/// EBADMSG: the file to import is not the one its digest pins.
pub(super) const ERRNO_BADMSG: i64 = -74;
/// EAGAIN: no disk is chosen yet, the USB driver still looking; ask again.
const ERRNO_AGAIN: i64 = -11;

pub(super) fn errno(e: VolumeError) -> i64 {
    match e {
        VolumeError::BlockFs(BlockFsError::NotFound)
        | VolumeError::NoImport
        | VolumeError::NotPassphraseKeyed => ERRNO_NOENT,
        VolumeError::NameTaken
        | VolumeError::BlockFs(BlockFsError::DirectoryFull)
        | VolumeError::VolumeExists => ERRNO_EXIST,
        VolumeError::BlockFs(BlockFsError::OutOfSpace)
        | VolumeError::Stretch(Argon2Error::NoMemory) => ERRNO_NOMEM,
        VolumeError::NeedsPassphrase | VolumeError::WrongPassphrase => ERRNO_ACCES,
        VolumeError::AlreadyOpen => ERRNO_BUSY,
        VolumeError::Device(BlockDeviceError::NotReady) => ERRNO_AGAIN,
        VolumeError::DigestMismatch => ERRNO_BADMSG,
        _ => ERRNO_IO,
    }
}
