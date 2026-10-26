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

//! What a process's death does to the calls around it, with no lock and no
//! scheduler, so the rule is proven on the host. A dead server owes every
//! call queued on it an answer it will never give: each caller is named, to
//! be told at once rather than left to sleep out its timeout. A dead caller's
//! own entries go from every queue, since nothing will read their replies.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

use super::entry::Entry;

/// Each server's queue of the calls owed a reply, oldest first.
pub(super) type Queues = BTreeMap<u32, VecDeque<Entry>>;

/// Clears `dead` from `queues` and returns the (caller pid, token) of every
/// call it still owed, oldest first, a dead caller's own calls left out.
pub(super) fn on_death(queues: &mut Queues, dead: u32, dead_inbox: &str) -> Vec<(u32, u64)> {
    let owed = queues.remove(&dead).unwrap_or_default();
    queues.retain(|_, queue| {
        queue.retain(|e| e.inbox != dead_inbox);
        !queue.is_empty()
    });
    owed.into_iter().filter(|e| e.pid != dead).map(|e| (e.pid, e.token)).collect()
}
