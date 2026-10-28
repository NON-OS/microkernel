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

//! Opening a pseudo-terminal: /dev/ptmx makes a pair and gives its master,
//! as posix_openpt asks; /dev/pts/N its slave once unlockpt has unlocked
//! it; /dev/tty the slave the caller's session has as its terminal.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::cell::with;
use super::make::{end, master};
use super::name::pts_number;

const O_NOCTTY: u64 = 0o400;

/// The answer to opening `full`, or None when it names no pseudo-terminal.
pub fn open(guest: &mut Guest, full: &[u8], flags: u64) -> Option<u64> {
    if full == b"/dev/ptmx" {
        return Some(master(guest, flags));
    }
    let n = match full {
        b"/dev/tty" => with(|t| t.controlling(guest.sid))?,
        _ => pts_number(full)?,
    };
    Some(slave(guest, n, flags))
}

/// The slave of pair `n`. A session leader with no controlling terminal
/// takes it as one unless O_NOCTTY says not to, as Linux's tty_open does.
pub fn slave(guest: &mut Guest, n: u32, flags: u64) -> u64 {
    if let Err(e) = with(|t| t.may_open(n)) {
        return errno::fail(e);
    }
    let Some(fd) = end(guest, n, false, flags) else {
        return errno::fail(errno::EMFILE);
    };
    let leader = guest.pid == guest.sid && flags & O_NOCTTY == 0;
    with(|t| {
        let takes = leader && t.controlling(guest.sid).is_none();
        if let Some(p) = t.get(n) {
            (p.slave, p.slave_seen) = (true, true);
            if takes && p.session == 0 {
                (p.session, p.fg) = (guest.sid, crate::linux::serve::guest_pid(guest.pgid));
            }
        }
    });
    errno::ok(fd)
}
