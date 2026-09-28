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

//! `wait4`: a child that has ended, or a wait until one does.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// Set by a caller that will not wait.
const WNOHANG: u64 = 1;

pub fn wait4(guest: &mut Guest, want: u64, status: u64, flags: u64, tid: u32) -> Answer {
    if guest.children.is_empty() {
        return Answer::value(errno::fail(errno::ECHILD));
    }
    if let Some(v) = reap_one(guest, want, status) {
        return Answer::value(v);
    }
    // A child still running under WNOHANG is a zero, not an error.
    if flags & WNOHANG != 0 {
        return Answer::value(errno::ok(0));
    }
    guest.waiting = Some((want, status, tid));
    Answer::Park
}

/// Take one ended child the caller asked about, write its status, and give
/// the answer wait4 returns. None while no such child has ended.
pub fn reap_one(guest: &mut Guest, want: u64, status: u64) -> Option<u64> {
    let any = (want as i64) <= 0;
    let at = guest.ended.iter().position(|(pid, _)| any || *pid == want as u32)?;
    let (pid, kept) = guest.ended.remove(at);
    guest.children.retain(|p| *p != pid);
    /* Kept as Linux's wait status word: an exit's code in the second byte. */
    let word = kept as u32;
    if status != 0 && guest.write(status, &word.to_le_bytes()) < 4 {
        return Some(errno::fail(errno::EFAULT));
    }
    Some(errno::ok(pid as u64))
}
