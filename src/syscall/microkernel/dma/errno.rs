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

//! A refused DMA map as the errno its caller sees.

use super::super::errnos::{
    ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOMEM, ERRNO_NOTSUP, ERRNO_PERM, ERRNO_RANGE, ERRNO_STALE,
};
use crate::hardware::broker::DmaMapError;

pub(super) fn errno_for(e: DmaMapError) -> i64 {
    match e {
        DmaMapError::NotClaimed | DmaMapError::Unconfined => ERRNO_PERM,
        DmaMapError::StaleEpoch => ERRNO_STALE,
        DmaMapError::UnknownDevice => ERRNO_NODEV,
        DmaMapError::BadAlignment | DmaMapError::BadLength | DmaMapError::BadLengthForClass => {
            ERRNO_INVAL
        }
        DmaMapError::UnsupportedFlags => ERRNO_NOTSUP,
        DmaMapError::NoMemory | DmaMapError::NoVaSpace | DmaMapError::MapFailed => ERRNO_NOMEM,
        // A 32-bit device would truncate the address: said apart from plain
        // exhaustion so the driver can name it.
        DmaMapError::Above4G => ERRNO_RANGE,
    }
}
