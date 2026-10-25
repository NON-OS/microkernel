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

//! The data volume, or the memory a live session holds it in.

use super::{said, Reason};

/// No data volume (ENODEV): no disk carries NONOS at all.
pub const WHY_NO_VOLUME: u8 = 10;
/// The data volume is locked until its passphrase is given.
pub const WHY_VOLUME_LOCKED: u8 = 11;
/// The data volume has no room for the model (ENOSPC).
pub const WHY_NO_ROOM: u8 = 16;
/// The data volume is there and could not be reached: EIO, EAGAIN.
pub const WHY_VOLUME_FAILED: u8 = 20;
/// Too little free memory to hold a live session's data volume (ENOMEM).
pub const WHY_NO_MEMORY: u8 = 25;

pub(super) fn volume_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_NO_VOLUME => said(
            "A live boot with no NONOS disk at all has nowhere to hold a model. \
             Install NONOS: Install is in the dock",
            false,
        ),
        WHY_VOLUME_LOCKED => said("The data volume is locked; unlock it, then retry", true),
        WHY_NO_ROOM => said(
            "No room for the model. On a live session it is held in memory: close programs \
             to free memory and retry, or install NONOS",
            true,
        ),
        /*
         * Honest for both cases the kernel answers EIO in today: a disk that
         * failed, and a machine with no disk carrying NONOS at all.
         */
        WHY_VOLUME_FAILED => said(
            "The data volume could not be reached. Check the disk, then retry; \
             with no NONOS disk at all there is nowhere to hold a model",
            true,
        ),
        WHY_NO_MEMORY => said(
            "Too little free memory to hold models on this live session: install NONOS \
             to keep them on a disk",
            false,
        ),
        _ => return None,
    })
}
