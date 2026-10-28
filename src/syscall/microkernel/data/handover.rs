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

//! `MkDataHandover()`: prepare the data volume for an update, its key
//! sealed once more for the first boot of the next release's bootloader
//! (fs/blockfs_volume/handover/). The installer asks it on the person's
//! word. Needs StoreWrite and FileSystem, as keying the volume does, and
//! opens the volume first if nothing has. Returns 0 once the record reads
//! back; ENOTSUP on a boot that keeps nothing, EAGAIN while the volume
//! cannot open yet, EALREADY for a volume a passphrase keys, ENOENT for one
//! keyed for one kernel, EACCES when this boot's chain is not approved or
//! its key does not open the volume, EIO when the disk did not take it.

use super::super::errnos::{
    ERRNO_ACCES, ERRNO_AGAIN, ERRNO_ALREADY, ERRNO_NOENT, ERRNO_NOTSUP, ERRNO_PERM,
};
use super::errno::errno;
use crate::fs::blockfs_volume::HandoverRefusal as R;
use crate::fs::blockfs_volume::{open_machine_volume, prepare_handover, VolumeError};

pub fn sys_data_handover() -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    if !(caps.can_store_write() && caps.can_open_files()) {
        return ERRNO_PERM;
    }
    match open_machine_volume().and_then(|_| prepare_handover()) {
        Ok(()) => 0,
        Err(VolumeError::NeedsPassphrase) | Err(VolumeError::Handover(R::NoNeed)) => ERRNO_ALREADY,
        Err(VolumeError::Handover(R::NotKept)) => ERRNO_NOTSUP,
        Err(VolumeError::Handover(R::NotOpen)) => ERRNO_AGAIN,
        Err(VolumeError::Handover(R::CannotKeep)) => ERRNO_NOENT,
        Err(VolumeError::Handover(R::NotApproved)) => ERRNO_ACCES,
        Err(e) => errno(e),
    }
}
