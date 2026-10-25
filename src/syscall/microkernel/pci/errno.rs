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

//! The errno each refused PCI config read or write answers.

use super::super::errnos::{ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM, ERRNO_STALE};
use crate::hardware::broker::{PciReadError, PciWriteError};

pub(super) fn read_errno(e: PciReadError) -> i64 {
    match e {
        PciReadError::NotClaimed => ERRNO_PERM,
        PciReadError::StaleEpoch => ERRNO_STALE,
        PciReadError::NoDeviceHandle | PciReadError::PlatformError => ERRNO_NODEV,
        PciReadError::OffsetNotAllowed | PciReadError::WidthNotAllowed => ERRNO_INVAL,
    }
}

pub(super) fn write_errno(e: PciWriteError) -> i64 {
    match e {
        PciWriteError::NotClaimed
        | PciWriteError::BusMasterWithoutDma
        | PciWriteError::MemorySpaceWithoutMmio => ERRNO_PERM,
        PciWriteError::StaleEpoch => ERRNO_STALE,
        PciWriteError::NoDeviceHandle | PciWriteError::PlatformError => ERRNO_NODEV,
        PciWriteError::OffsetNotAllowed | PciWriteError::BitsNotAllowed => ERRNO_INVAL,
    }
}
