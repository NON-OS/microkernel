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

/* What a close and an exit take away: POSIX locks and flock's. */

use crate::linux::guest::Guest;

use super::super::super::desc;
use super::super::lock::{self, Owner};
use super::flock::file_of;

/*
 * What a close of `fd` releases: every POSIX lock the process holds on the
 * file, and a description's flock and OFD locks once no other descriptor
 * anywhere in the family holds that description.
 */
pub fn closing(guest: &Guest, fd: u64) {
    let Ok((f, d)) = file_of(guest, fd) else { return };
    let (file, me) = (f.path.clone(), guest.pid);
    let last = !desc::held_elsewhere(guest, fd, d);
    lock::drop_where(|l| {
        l.file == file
            && match l.owner {
                Owner::Posix(pid) => pid == me,
                Owner::Flock(x) | Owner::Ofd(x) => x == d && last,
            }
    });
    if last {
        desc::gone(d);
    }
}
