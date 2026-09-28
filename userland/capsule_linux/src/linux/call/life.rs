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

//! Ending a guest. The call never returns to the guest, so the answer
//! handed back is only what parks it until the supervisor tears it down.

use nonos_libc::mk_kill;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

const SIGKILL: u64 = 9;

/// A thread's own exit ends that thread and never returns to it. The word it
/// named with CLONE_CHILD_CLEARTID or set_tid_address is zeroed and one waiter
/// woken, as Linux does; that is what a joiner waits for. Left unanswered, the
/// thread is killed, so it cannot run past its exit.
pub fn exit_thread(guest: &mut Guest, tid: u32) -> Answer {
    if let Some(at) = guest.clear_tids.iter().position(|(t, _)| *t == tid) {
        let (_, word) = guest.clear_tids.remove(at);
        if guest.write(word, &0u32.to_le_bytes()) == 4 {
            guest.wake(word, 1);
        }
    }
    guest.threads.retain(|t| *t != tid);
    let rc = mk_kill(u64::from(tid), SIGKILL);
    if rc < 0 {
        let line = alloc::format!(
            "[LINUX] kill refused: exited thread {tid} stays parked, errno {}\n",
            -rc
        );
        crate::linux::start::say(line.as_bytes());
    }
    Answer::Park
}

/// set_tid_address names the word to clear when the calling thread exits, and
/// answers with its tid.
pub fn set_tid_address(guest: &mut Guest, tid: u32, word: u64) -> Answer {
    guest.clear_tids.retain(|(t, _)| *t != tid);
    if word != 0 {
        guest.clear_tids.push((tid, word));
    }
    Answer::value(u64::from(tid))
}

pub fn exit(guest: &mut Guest, code: u64) -> u64 {
    guest.exited = Some(code as i32);
    errno::ok(0)
}
