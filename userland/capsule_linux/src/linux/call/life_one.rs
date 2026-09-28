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

//! A plain exit, which ends the calling thread only. The process ends with
//! the last of its threads, and that thread's code is its status.

use super::life::{exit, exit_thread};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// A plain exit ends the calling thread only; the process ends with the last
/// of its threads, and that thread's code is its status. The leader cannot be
/// killed while others run: its pid is the address space every peer call
/// names. So it stays parked inside its exit, a zombie that runs nothing,
/// until the process ends and takes it.
pub fn exit_one(guest: &mut Guest, tid: u32, code: u64) -> Answer {
    if tid == guest.pid {
        if let Some(at) = guest.clear_tids.iter().position(|(t, _)| *t == tid) {
            let (_, word) = guest.clear_tids.remove(at);
            if guest.write(word, &0u32.to_le_bytes()) == 4 {
                guest.wake(word, 1);
            }
        }
        guest.signals.leader_gone = true;
    } else {
        let _ = exit_thread(guest, tid);
    }
    guest.forget_thread(tid);
    if guest.live_threads().is_empty() {
        let _ = exit(guest, code);
    }
    Answer::Park
}
