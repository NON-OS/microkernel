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

//! Parking a guest's call with its family's sequence number.

use super::super::frame::ForeignFrame;
use super::super::trap_table::park as park_frame;
use super::table::with_family;

/*
 * The number is stamped before the frame is parked: once parked, the
 * supervisor can claim and answer it at once, and an answer must find its
 * call open. A park that fails leaves the guest ENOENT, so the call closes
 * as abandoned, refused.
 */
pub(in crate::process::foreign) fn park(supervisor: u32, frame: ForeignFrame) -> bool {
    let (pid, nr) = (frame.pid, frame.nr);
    with_family(supervisor, |f| f.trapped(pid, nr));
    if park_frame(frame) {
        return true;
    }
    with_family(supervisor, |f| f.park_failed(pid, nr));
    false
}

/// The early refusal in redirect: a number no supervisor is handed.
pub(in crate::process::foreign) fn refused_early(nr: u64) -> Option<u64> {
    super::absorb::refused(nr, -38i64 as u64);
    None
}
