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

//! Joining and leaving one futex's waiter list. Pure, so the order a wait
//! takes them in can be checked on the host against every waker interleaving.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// (thread group, address): the threads of one capsule share a futex.
pub(super) type Key = (u32, u64);

pub(super) fn join(queue: &mut BTreeMap<Key, Vec<u32>>, key: Key, pid: u32) {
    queue.entry(key).or_default().push(pid);
}

/// Leave the list if a waker has not already taken this pid off it; an empty
/// list is dropped so the map holds only futexes someone waits on.
pub(super) fn leave(queue: &mut BTreeMap<Key, Vec<u32>>, key: Key, pid: u32) {
    if let Some(waiting) = queue.get_mut(&key) {
        waiting.retain(|&p| p != pid);
        if waiting.is_empty() {
            queue.remove(&key);
        }
    }
}

/// Leave on a return that does not sleep: the word had already changed. A
/// pid a waker took off the list had that wake spent on it while it was
/// returning anyway, and a waiter asleep behind it then slept out its whole
/// timeout. Linux never spends a wake that way, since futex_wait_setup reads
/// the word with the bucket locked. So the next waiter is taken in its place
/// and handed back for the caller to wake.
pub(super) fn leave_early(queue: &mut BTreeMap<Key, Vec<u32>>, key: Key, pid: u32) -> Option<u32> {
    if queue.get(&key).is_some_and(|waiting| waiting.contains(&pid)) {
        leave(queue, key, pid);
        return None;
    }
    let waiting = queue.get_mut(&key)?;
    let next = (!waiting.is_empty()).then(|| waiting.remove(0));
    if waiting.is_empty() {
        queue.remove(&key);
    }
    next
}
