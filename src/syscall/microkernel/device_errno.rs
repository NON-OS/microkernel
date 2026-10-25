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

//! A device claim's and release's refusals as errnos. Resetting (L78) is
//! EAGAIN: a wipe under way ends within a bound, and libc asks again. A
//! device outside the claimant's scope row is EPERM; one whose group
//! another capsule holds is EBUSY. A record marked unusable is no
//! device to claim; the firmware's flash is one no capsule may have.

use super::super::errnos::{ERRNO_AGAIN, ERRNO_BUSY, ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM};
use crate::hardware::broker::ClaimError;

pub(super) fn claim_errno(e: ClaimError) -> i64 {
    match e {
        ClaimError::AlreadyClaimed | ClaimError::Held => ERRNO_BUSY,
        ClaimError::Busy | ClaimError::GroupHeld => ERRNO_BUSY,
        ClaimError::UnknownDevice | ClaimError::Disabled => ERRNO_NODEV,
        ClaimError::NotHolder | ClaimError::NotClaimed => ERRNO_INVAL,
        ClaimError::Unconfined | ClaimError::Refused | ClaimError::Firmware => ERRNO_PERM,
        ClaimError::OutOfScope => ERRNO_PERM,
        ClaimError::Resetting => ERRNO_AGAIN,
    }
}

pub(super) fn release_errno(e: ClaimError) -> i64 {
    match e {
        ClaimError::NotClaimed => ERRNO_NODEV,
        ClaimError::NotHolder => ERRNO_PERM,
        ClaimError::AlreadyClaimed
        | ClaimError::UnknownDevice
        | ClaimError::Unconfined
        | ClaimError::Disabled
        | ClaimError::Firmware
        | ClaimError::Held
        | ClaimError::Refused
        | ClaimError::Resetting
        | ClaimError::OutOfScope
        | ClaimError::Busy
        | ClaimError::GroupHeld => ERRNO_INVAL,
    }
}
