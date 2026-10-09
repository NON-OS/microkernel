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

/* A fed import's refusal as the errno a caller sees. */

use super::super::errnos::{ERRNO_BUSY, ERRNO_FBIG, ERRNO_INPROGRESS, ERRNO_INVAL, ERRNO_NOSPC};
use super::errno::errno;
use crate::fs::blockfs_volume::StreamError;

pub(super) fn feed_errno(e: StreamError) -> i64 {
    match e {
        StreamError::Volume(v) => errno(v),
        StreamError::Busy => ERRNO_BUSY,
        StreamError::NotBegun => ERRNO_INVAL,
        StreamError::Overrun => ERRNO_FBIG,
        StreamError::Short => ERRNO_INPROGRESS,
        StreamError::NoRoom => ERRNO_NOSPC,
    }
}
