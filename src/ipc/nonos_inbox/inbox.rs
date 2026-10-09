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

//! Per-module message inbox.

extern crate alloc;

use alloc::collections::VecDeque;
use core::sync::atomic::Ordering;
use spin::Mutex;

use super::budget::{self, Held};
use super::stats::{InboxStats, InboxStatsSnapshot};
use crate::ipc::nonos_channel::IpcMessage;

/*
 * The bytes waiting in every inbox. Taken only inside an inbox's queue lock
 * (that first, then this), and the timer's teardown, which drops inboxes,
 * runs only while no kernel lock is held (`timer_trampoline/reclaim.rs`), so
 * a plain spin lock serves.
 */
static TOTAL: Mutex<usize> = Mutex::new(0);

/// What `msg` is charged while it waits: its payload, both names, and the
/// fixed overhead.
fn charge(msg: &IpcMessage) -> usize {
    budget::cost(msg.data.len().saturating_add(msg.from.len()).saturating_add(msg.to.len()))
}

/// The queued messages and what they are charged, under one lock so the two
/// never disagree.
struct Queue {
    msgs: VecDeque<IpcMessage>,
    held: Held,
}

/// Per-module message inbox with bounded capacity. `owner` is the
/// pid that registered the inbox; `0` is kernel-owned (the reply
/// inboxes that capsule_spawn pre-registers). A non-zero owner is
/// liveness-checked on every strict enqueue so the kernel cannot
/// route a message to a queue whose draining capsule has exited.
pub(super) struct Inbox {
    queue: Mutex<Queue>,
    capacity: usize,
    owner: u32,
    stats: InboxStats,
}

impl Inbox {
    pub(super) fn new(capacity: usize, owner: u32) -> Self {
        let queue = Queue { msgs: VecDeque::with_capacity(capacity), held: Held::new() };
        Self { queue: Mutex::new(queue), capacity, owner, stats: InboxStats::new() }
    }

    #[inline]
    pub(super) fn owner(&self) -> u32 {
        self.owner
    }

    /// Check if inbox is full
    #[inline]
    pub(super) fn is_full(&self) -> bool {
        self.queue.lock().msgs.len() >= self.capacity
    }

    /// Check if inbox is empty
    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        self.queue.lock().msgs.is_empty()
    }

    /// Get current queue length
    #[inline]
    pub(super) fn len(&self) -> usize {
        self.queue.lock().msgs.len()
    }

    /// Get inbox capacity
    #[inline]
    pub(super) fn capacity(&self) -> usize {
        self.capacity
    }

    /// Try to enqueue without blocking. Refused when the inbox holds
    /// `capacity` messages or the message does not fit the byte budget.
    pub(super) fn try_enqueue(&self, msg: IpcMessage) -> Result<(), IpcMessage> {
        let mut q = self.queue.lock();
        let sender = budget::sender_of(&msg.from);
        if q.msgs.len() < self.capacity && q.held.admit(&mut TOTAL.lock(), sender, charge(&msg)) {
            q.msgs.push_back(msg);
            let size = q.msgs.len();
            drop(q);
            self.stats.record_enqueue(size);
            Ok(())
        } else {
            self.stats.record_dropped();
            Err(msg)
        }
    }

    /// Dequeue next message
    #[inline]
    pub(super) fn dequeue(&self) -> Option<IpcMessage> {
        let mut q = self.queue.lock();
        let msg = q.msgs.pop_front()?;
        q.held.release(&mut TOTAL.lock(), budget::sender_of(&msg.from), charge(&msg));
        drop(q);
        self.stats.record_dequeue();
        Some(msg)
    }

    /// Peek at next message without removing
    pub(super) fn peek(&self) -> Option<IpcMessage> {
        self.queue.lock().msgs.front().cloned()
    }

    /// Get statistics snapshot
    pub(super) fn get_stats(&self) -> InboxStatsSnapshot {
        InboxStatsSnapshot {
            enqueued: self.stats.enqueued.load(Ordering::Relaxed),
            dequeued: self.stats.dequeued.load(Ordering::Relaxed),
            dropped_full: self.stats.dropped_full.load(Ordering::Relaxed),
            timeouts: self.stats.timeouts.load(Ordering::Relaxed),
            peak_size: self.stats.peak_size.load(Ordering::Relaxed),
            current_size: self.len(),
            bytes: self.queue.lock().held.bytes(),
            capacity: self.capacity,
        }
    }

    /// Clear all messages from inbox
    pub(super) fn clear(&self) -> usize {
        let mut q = self.queue.lock();
        let count = q.msgs.len();
        q.msgs.clear();
        q.held.release_all(&mut TOTAL.lock());
        count
    }
}

/// An inbox unregistered or replaced with messages still queued gives their
/// bytes back to the total.
impl Drop for Inbox {
    fn drop(&mut self) {
        self.queue.get_mut().held.release_all(&mut TOTAL.lock());
    }
}
