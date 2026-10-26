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

//! The stream lock, held over work on the volume. It is always taken before
//! the volume's, as a feed takes them.

use super::super::error::VolumeError;
use super::live::LIVE;

/*
 * `f`, run while no stream is coming to `name`, with the stream lock held so
 * none can begin under it; Importing when one is. The stream lock is always
 * taken before the volume's, as a feed takes them.
 */
pub(in super::super) fn without_stream<R>(
    name: &[u8],
    f: impl FnOnce() -> Result<R, VolumeError>,
) -> Result<R, VolumeError> {
    let live = crate::smp::lock_responsive(&LIVE);
    if live.as_ref().is_some_and(|l| &l.name[..l.name_len] == name) {
        return Err(VolumeError::Importing);
    }
    let done = f();
    drop(live);
    done
}

/*
 * `f` with the stream lock held, so no import is part way through it; the
 * stream in flight ends when `f` says so, and its next feed finds none.
 */
pub(in super::super) fn with_streams<R>(f: impl FnOnce() -> (R, bool)) -> R {
    let mut live = crate::smp::lock_responsive(&LIVE);
    let (done, end) = f();
    if end {
        *live = None;
    }
    done
}
