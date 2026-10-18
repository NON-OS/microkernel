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

use crate::hardware::broker::{IrqBindError, IrqError};
use crate::syscall::microkernel::errnos::{
    ERRNO_BUSY, ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOMEM, ERRNO_NOTSUP, ERRNO_PERM, ERRNO_STALE,
};

pub(super) fn bind_errno(e: IrqBindError) -> i64 {
    match e {
        IrqBindError::NotClaimed => ERRNO_PERM,
        IrqBindError::StaleEpoch => ERRNO_STALE,
        IrqBindError::UnknownDevice | IrqBindError::NoDeviceHandle => ERRNO_NODEV,
        IrqBindError::NotDeviceIrq
        | IrqBindError::NotIntx
        | IrqBindError::NoMsixCap
        | IrqBindError::NoMsiCap
        | IrqBindError::BadMsixBar
        | IrqBindError::BadVectorCount => ERRNO_INVAL,
        // Held by the kernel, not by another capsule, and it is never
        // released. Reported apart from AlreadyBound so a driver is not told
        // to wait for a line that will not come free.
        IrqBindError::ReservedGsi => ERRNO_PERM,
        // Refused by policy: retrying never helps.
        IrqBindError::NotRemapped => ERRNO_PERM,
        IrqBindError::AlreadyBound => ERRNO_BUSY,
        IrqBindError::NoVector => ERRNO_NOMEM,
        IrqBindError::UnsupportedFlags => ERRNO_NOTSUP,
        IrqBindError::MsixProgramFailed
        | IrqBindError::MsiProgramFailed
        | IrqBindError::PlatformError => ERRNO_NODEV,
    }
}

/// As `grant_errno`, but a grant a depart took answers ENODEV: the device
/// is gone, which is not the caller's mistake.
pub(super) fn grant_errno_for(pid: u32, grant_id: u64, e: IrqError) -> i64 {
    match e {
        IrqError::UnknownGrant if crate::hardware::broker::irq::gone::departed(pid, grant_id) => {
            ERRNO_NODEV
        }
        e => grant_errno(e),
    }
}

/// The errno for a refused wait on a grant.
pub(super) fn grant_errno(e: IrqError) -> i64 {
    match e {
        IrqError::NotHolder => ERRNO_PERM,
        IrqError::UnknownGrant => ERRNO_INVAL,
        IrqError::PlatformError => ERRNO_NODEV,
    }
}
