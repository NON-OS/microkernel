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

//! A block device refusal as the errno the store calls answer with. MkStoreRead
//! and MkStoreWrite both map through here, so a read and a write of the same
//! disk agree: no disk is ENODEV, a refused caller EACCES, a driver still
//! coming up or gone quiet ETIMEDOUT, a request outside the disk EINVAL. The
//! rest keep the EFAULT the store's callers already read as a device fault.
//! Pure, so the table is held on the host (kernel_proofs).

use crate::hardware::block_device::BlockDeviceError as B;
use crate::syscall::microkernel::errnos::{
    ERRNO_ACCES, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_TIMEDOUT,
};

pub(super) const fn store_errno(e: B) -> i64 {
    match e {
        B::Dead => ERRNO_NODEV,
        B::AccessDenied | B::NoCallerPid => ERRNO_ACCES,
        B::Stale | B::TransportFailure | B::NotReady => ERRNO_TIMEDOUT,
        B::OutOfRange | B::InvalidArgument => ERRNO_INVAL,
        B::OversizedRequest
        | B::DeviceFailure
        | B::Unsupported
        | B::ProtocolMismatch
        | B::BlockSize => ERRNO_FAULT,
    }
}
