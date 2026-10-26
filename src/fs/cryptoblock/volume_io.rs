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

//! The token for the data volume's own disk I/O (block_device's
//! volume_scope). Its field is private here, so only this module makes one,
//! and only crate::fs reaches `volume_scope`: the volume's reads, writes,
//! plan and key header, for a data syscall that checked its caller.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::hardware::block_device::{BlockDeviceError, VolumeScope};

pub struct VolumeIo(());

static SAID: AtomicBool = AtomicBool::new(false);

/// Enter the scope for one I/O; hold it until the I/O is done. With every
/// slot taken the I/O is refused (NotReady, EAGAIN to the caller), said once.
pub(in crate::fs) fn volume_scope() -> Result<VolumeScope, BlockDeviceError> {
    crate::hardware::block_device::volume_scope(&VolumeIo(())).ok_or_else(|| {
        if !SAID.swap(true, Ordering::Relaxed) {
            crate::sys::serial::print(
                b"[DATA] volume I/O refused: every volume scope is taken; try again\n",
            );
        }
        BlockDeviceError::NotReady
    })
}
