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

//! A disk plan refusal as the errno a caller sees. A disk with no plan on it
//! is a machine with no data volume, ENODEV, which the store and the model
//! fetcher tell the person in those words. A plan that names ranges it may
//! not is a disk to distrust, EIO, the same as a disk that fails.

use crate::fs::blockfs_volume::PlanError;
use crate::syscall::microkernel::errnos::{ERRNO_IO, ERRNO_NODEV};

pub(super) const fn plan_errno(e: PlanError) -> i64 {
    match e {
        PlanError::NoPlan => ERRNO_NODEV,
        PlanError::BelowFloor
        | PlanError::VolumeTooSmall
        | PlanError::PastEnd
        | PlanError::Overlap
        | PlanError::BadImport => ERRNO_IO,
    }
}
