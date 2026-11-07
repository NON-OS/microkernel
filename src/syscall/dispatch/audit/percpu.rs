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

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use spin::{Mutex, Once};

/// Records each CPU keeps: what the one shared ring kept for all of them.
pub(super) const RING: usize = 256;

/*
 * The entries live on the heap and are written there one at a time, never
 * assembled on a stack first. A ring of kernel records is 22 KiB and is made
 * on a CPU's first syscall, on the caller's 32 KiB kernel stack, and kernel
 * stacks sit next to each other with no guard page: a ring built there and
 * then moved overran that stack into the next process's, over its saved
 * return addresses. A clear empties the
 * entries where they are, for the same reason.
 */
struct Ring<T> {
    entries: Box<[Option<(u64, T)>]>,
    head: usize,
}

impl<T> Ring<T> {
    /// None when the heap has no room for the entries.
    fn new() -> Option<Self> {
        let mut entries = Vec::new();
        entries.try_reserve_exact(RING).ok()?;
        entries.resize_with(RING, || None);
        Some(Self { entries: entries.into_boxed_slice(), head: 0 })
    }

    fn push(&mut self, seq: u64, entry: T) {
        self.entries[self.head] = Some((seq, entry));
        self.head = (self.head + 1) % RING;
    }

    fn empty(&mut self) {
        self.entries.iter_mut().for_each(|e| *e = None);
        self.head = 0;
    }
}

/*
 * One ring per CPU, each behind its own lock, which only that CPU takes on
 * the syscall path (a reader takes each in turn), so the record no longer
 * serialises every CPU's IPC and futex calls. A global sequence number,
 * taken before the record is stored, orders records across CPUs for the
 * reader; within a CPU it is the order of the pushes.
 *
 * Nothing the shared ring kept is lost: a record among the last RING of all
 * CPUs has fewer than RING later records on its own CPU, so its own ring
 * still holds it. A ring is allocated on its CPU's first record; while the
 * heap cannot hold it, that CPU's records are dropped and the next record
 * tries again.
 */
pub(super) struct PerCpuLog<T, const N: usize> {
    rings: [Once<Mutex<Ring<T>>>; N],
    seq: AtomicU64,
}

impl<T: Clone, const N: usize> PerCpuLog<T, N> {
    pub(super) const fn new() -> Self {
        Self { rings: [const { Once::new() }; N], seq: AtomicU64::new(0) }
    }

    /// False when the record was dropped: its CPU has no ring and the heap
    /// had no room for one.
    pub(super) fn push(&self, cpu: usize, entry: T) -> bool {
        let Ok(ring) = self.rings[cpu % N].try_call_once(|| Ring::new().map(Mutex::new).ok_or(()))
        else {
            return false;
        };
        let mut ring = ring.lock();
        let seq = self.seq.fetch_add(1, Ordering::Relaxed);
        ring.push(seq, entry);
        true
    }

    /// The `max` most recent records, newest first.
    pub(super) fn recent(&self, max: usize) -> Vec<T> {
        let mut all: Vec<(u64, T)> = Vec::new();
        for slot in &self.rings {
            if let Some(ring) = slot.get() {
                let ring = ring.lock();
                all.extend(ring.entries.iter().flatten().cloned());
            }
        }
        all.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        all.truncate(max);
        all.into_iter().map(|(_, e)| e).collect()
    }

    pub(super) fn clear(&self) {
        for slot in &self.rings {
            if let Some(ring) = slot.get() {
                ring.lock().empty();
            }
        }
    }
}
