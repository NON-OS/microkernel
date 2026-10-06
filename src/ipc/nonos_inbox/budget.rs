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

//! How many bytes the kernel holds queued in IPC inboxes, and the rule a
//! message must pass to be held.
//!
//! An inbox counted messages only: 1024 of up to 1 MiB each is a gigabyte
//! held in a 256 MiB kernel heap, and a heap that runs out halts the
//! machine. Any capsule that could send to an inbox, its own included, could
//! do that. Now every inbox together holds at most TOTAL_BYTES_MAX, one inbox
//! at most INBOX_BYTES_MAX, and one sender at most SHARE_BYTES_MAX of any one
//! inbox, so a sender that floods a service leaves the other half of its
//! inbox to everyone else. A message past any of them is refused as a full
//! queue (EAGAIN or EBUSY to the sender), which a sender already meets when
//! an inbox holds its 1024 messages.
//!
//! There is deliberately no cap on what one sender has waiting across every
//! inbox: replies wait in the inbox of the client that asked, so such a cap
//! would let a client that never reads its replies spend a server's whole
//! allowance and cut every other client off. A flooder reaches only the
//! inboxes its peer list allows and holds at most half of each. The kernel's
//! own messages (sender 0) count against the inbox and the total only.

extern crate alloc;

use alloc::collections::BTreeMap;

pub(crate) const TOTAL_BYTES_MAX: usize = 96 << 20;
pub(crate) const INBOX_BYTES_MAX: usize = 16 << 20;
pub(crate) const SHARE_BYTES_MAX: usize = INBOX_BYTES_MAX / 2;
/// What a queued message holds beyond its payload and names: its envelope
/// and its place in the queue.
pub(crate) const MESSAGE_OVERHEAD: usize = 128;

/// What a message carrying `bytes` (payload and both names) is charged while
/// it waits.
pub(crate) fn cost(bytes: usize) -> usize {
    bytes.saturating_add(MESSAGE_OVERHEAD)
}

/// The pid the kernel stamped on a message as `proc.<pid>`, or 0 for a
/// message the kernel sent under a name of its own.
pub(crate) fn sender_of(from: &str) -> u32 {
    from.strip_prefix("proc.").and_then(|s| s.parse::<u32>().ok()).map_or(0, |pid| pid)
}

/// The bytes one inbox holds, and how many of them each sender sent. `total`
/// in every call is the bytes waiting in every inbox, which each call keeps
/// in step with this inbox.
pub(crate) struct Held {
    bytes: usize,
    senders: BTreeMap<u32, usize>,
}

impl Held {
    pub(crate) const fn new() -> Self {
        Self { bytes: 0, senders: BTreeMap::new() }
    }

    /// Take a message costing `cost` from `sender`, or refuse it and change
    /// nothing.
    pub(crate) fn admit(&mut self, total: &mut usize, sender: u32, cost: usize) -> bool {
        let fits = |held: usize, max: usize| held.checked_add(cost).is_some_and(|n| n <= max);
        let share = self.held_by(sender);
        if !fits(*total, TOTAL_BYTES_MAX) || !fits(self.bytes, INBOX_BYTES_MAX) {
            return false;
        }
        if sender != 0 && !fits(share, SHARE_BYTES_MAX) {
            return false;
        }
        *total += cost;
        self.bytes += cost;
        if sender != 0 {
            self.senders.insert(sender, share + cost);
        }
        true
    }

    /// A message from `sender` costing `cost` left this inbox.
    pub(crate) fn release(&mut self, total: &mut usize, sender: u32, cost: usize) {
        let cost = cost.min(self.bytes);
        *total = total.saturating_sub(cost);
        self.bytes -= cost;
        if let Some(share) = self.senders.get_mut(&sender) {
            *share = share.saturating_sub(cost);
            if *share == 0 {
                self.senders.remove(&sender);
            }
        }
    }

    /// Every message left at once: the inbox was cleared or dropped.
    pub(crate) fn release_all(&mut self, total: &mut usize) {
        *total = total.saturating_sub(self.bytes);
        self.bytes = 0;
        self.senders.clear();
    }

    pub(crate) fn bytes(&self) -> usize {
        self.bytes
    }

    pub(crate) fn held_by(&self, sender: u32) -> usize {
        self.senders.get(&sender).map_or(0, |share| *share)
    }
}
