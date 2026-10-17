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

//! Joining and leaving one futex's waiter list, pure so the host checks it.

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

/// Leave on a return that does not sleep: a wake spent on this pid goes to the
/// next live waiter, for the caller to wake, never to one killed (L81).
pub(super) fn leave_early(
    queue: &mut BTreeMap<Key, Vec<u32>>,
    key: Key,
    pid: u32,
    ended: impl Fn(u32) -> bool,
) -> Option<u32> {
    if queue.get(&key).is_some_and(|waiting| waiting.contains(&pid)) {
        leave(queue, key, pid);
        return None;
    }
    let waiting = queue.get_mut(&key)?;
    let next = take_live(waiting, 1, ended).pop();
    if waiting.is_empty() {
        queue.remove(&key);
    }
    next
}

/// Up to `count` live waiters from the front (0, all), any that ended dropped
/// uncounted: a wake spent on one killed after it queued was lost (L77e).
pub(super) fn take_live(
    waiting: &mut Vec<u32>,
    count: usize,
    ended: impl Fn(u32) -> bool,
) -> Vec<u32> {
    let mut woken = Vec::new();
    while (count == 0 || woken.len() < count) && !waiting.is_empty() {
        let pid = waiting.remove(0);
        if !ended(pid) {
            woken.push(pid);
        }
    }
    woken
}
