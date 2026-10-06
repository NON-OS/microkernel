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

/*
 * A lock that must wait: flock without LOCK_NB and F_SETLKW, which Linux
 * blocks until the lock in the way goes. Parked like a read on an empty
 * pipe (waits.rs), and tried again after every call the family makes, so
 * it is taken as soon as the close, unlock or exit that frees it is served.
 */

use nonos_libc::ForeignFrame;

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::file;
use crate::linux::guest::{Blocked, Guest};

use super::answer::Answer;

pub fn wants(frame: &ForeignFrame) -> bool {
    frame.nr == np::FLOCK || (frame.nr == nr::FCNTL && file::is_lock_cmd(frame.args()[1]))
}

pub fn lock(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let (tid, nr, a) = (frame.pid, frame.nr, frame.args());
    match try_lock(guest, nr, a) {
        file::LOCK_WAIT => {
            guest.blocked.push(Blocked { tid, nr, args: a, deadline: None, done: 0 });
            Answer::Park
        }
        v => Answer::value(v),
    }
}

/* A parked lock call tried again: its answer once it no longer waits. */
pub fn retry(guest: &mut Guest, wait: &Blocked) -> Option<u64> {
    Some(try_lock(guest, wait.nr, wait.args)).filter(|&v| v != file::LOCK_WAIT)
}

fn try_lock(guest: &mut Guest, nr: u64, a: [u64; 6]) -> u64 {
    match nr {
        np::FLOCK => file::flock(guest, a[0], a[1]),
        _ => file::fcntl_lock(guest, a[0], a[1], a[2]),
    }
}

/*
 * A lock call answered where nothing can park. The serve loop sends every
 * lock call to `lock` above; one that arrives here anyway and would wait
 * is said by name rather than answered with a value no caller knows.
 */
pub fn answer_now(v: u64) -> u64 {
    if v != file::LOCK_WAIT {
        return v;
    }
    let line = b"[LINUX] unserved lock wait: this path cannot park the caller\n";
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    errno::fail(errno::ENOLCK)
}
