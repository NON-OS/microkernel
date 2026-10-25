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

use super::state::PENDING;

/// `pid` died: every caller waiting on it is told at once (ESRCH, through
/// `abandoned`) and woken, and its own calls leave every queue. Teardown
/// calls this after the names go, so no call can find it once this has run.
pub(crate) fn clear_pid(pid: u32) {
    let caller_inbox = super::super::reply_inbox::for_pid(pid);
    // Gone recorded under PENDING, so no push can slip in after the clear.
    let mut queues = PENDING.lock();
    let owed = super::death::on_death(&mut queues, pid, &caller_inbox);
    super::abandoned::record(pid, &owed);
    drop(queues);
    for (caller, _) in owed {
        crate::sched::wake_process(caller);
    }
}
