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

use super::super::errnos::{
    ERRNO_ACCES, ERRNO_AGAIN, ERRNO_BADMSG, ERRNO_BUSY, ERRNO_EXIST, ERRNO_IO, ERRNO_NOENT,
    ERRNO_NODEV, ERRNO_NOMEM, ERRNO_NOSPC,
};
use crate::crypto::util::argon2::Argon2Error;
use crate::fs::blockfs::BlockFsError;
use crate::fs::blockfs_volume::VolumeError;
use crate::hardware::block_device::BlockDeviceError;

use super::plan_errno::plan_errno;

pub(super) fn errno(e: VolumeError) -> i64 {
    match e {
        VolumeError::BlockFs(BlockFsError::NotFound)
        | VolumeError::NoImport
        | VolumeError::NotPassphraseKeyed => ERRNO_NOENT,
        VolumeError::NameTaken
        | VolumeError::BlockFs(BlockFsError::DirectoryFull)
        | VolumeError::VolumeExists => ERRNO_EXIST,
        VolumeError::BlockFs(BlockFsError::OutOfSpace) => ERRNO_NOSPC,
        VolumeError::Stretch(Argon2Error::NoMemory) | VolumeError::NoMemory => ERRNO_NOMEM,
        VolumeError::NeedsPassphrase | VolumeError::WrongPassphrase => ERRNO_ACCES,
        VolumeError::AlreadyOpen | VolumeError::Importing => ERRNO_BUSY,
        VolumeError::Device(BlockDeviceError::NotReady) => ERRNO_AGAIN,
        /* No disk carries the store or a plan: there is no volume to open. */
        VolumeError::Device(BlockDeviceError::Dead) => ERRNO_NODEV,
        VolumeError::DigestMismatch => ERRNO_BADMSG,
        VolumeError::Plan(p) => plan_errno(p),
        // The catch-all: said with its cause, so an EIO is never a mystery.
        // The log's sinks are the screen only, so the line goes to serial too.
        other => {
            let line = alloc::format!("[DATA] refused with EIO: {:?}", other);
            crate::sys::serial::println(line.as_bytes());
            crate::log::warn!("{}", line);
            ERRNO_IO
        }
    }
}
