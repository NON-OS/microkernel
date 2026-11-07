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

/*
 * A single-producer, single-consumer ring of fixed-size records in shared
 * memory: client to compositor, client to broker.
 *
 * A record is `W` words. Slots are atomics, read and written Relaxed: a
 * hostile peer racing a slot can make a record garbage, never undefined
 * behaviour, and the receiver validates what it reads anyway.
 *
 * Ordering. The producer writes a record's words, then swaps the new head
 * in with Release; the consumer reads `head` with Acquire, so every word
 * written before the swap is visible once the new head is. The consumer
 * reads its records, then stores `tail` with Release; the producer loads
 * `tail` with Acquire before it reuses a slot, so a slot is never
 * overwritten while the consumer may still read it.
 *
 * Indices are free-running 31-bit counters (`INDEX`); bit 31 of the head
 * word is the doorbell arm. A slot is `index & (capacity - 1)`, so no index
 * from shared memory can reach outside the slots, and `head - tail`
 * (wrapping, 31 bits) above the capacity is a corrupt peer.
 *
 * Batching. `push` fills a slot without publishing; `publish` makes every
 * pushed record visible with one store. `drain` reads every published
 * record and releases them with one store.
 */

use crate::ring_error::RingError;
use crate::ring_header::RingHeader;
use crate::ring_sync::{AtomicU64, Ordering};

const MAX_CAPACITY: u32 = 1 << 30;

/// The index bits of `head` and `tail`.
pub(crate) const INDEX: u32 = 0x7fff_ffff;
/// Bit 31 of `head`: the consumer asked to be woken (`doorbell.rs`).
pub(crate) const ARMED: u32 = 0x8000_0000;

pub struct Ring<'a, const W: usize> {
    pub(crate) header: &'a RingHeader,
    slots: &'a [AtomicU64],
    pub(crate) mask: u32,
}

impl<'a, const W: usize> Ring<'a, W> {
    /// A ring over `slots`, which holds `capacity` records of `W` words.
    pub fn new(header: &'a RingHeader, slots: &'a [AtomicU64], capacity: u32) -> Result<Self, RingError> {
        let shaped = W > 0
            && capacity.is_power_of_two()
            && (2..=MAX_CAPACITY).contains(&capacity)
            && (capacity as usize).checked_mul(W) == Some(slots.len());
        if !shaped {
            return Err(RingError::Shape);
        }
        Ok(Self { header, slots, mask: capacity - 1 })
    }

    pub fn capacity(&self) -> u32 {
        self.mask + 1
    }

    /// The producer's end. The caller holds the only one.
    pub fn producer(&self) -> Producer<'_, 'a, W> {
        let head = self.header.head.load(Ordering::Relaxed) & INDEX;
        Producer { ring: self, head, published: head }
    }

    /// The consumer's end. The caller holds the only one.
    pub fn consumer(&self) -> Consumer<'_, 'a, W> {
        Consumer { ring: self, tail: self.header.tail.load(Ordering::Relaxed) & INDEX }
    }

    fn slot(&self, index: u32) -> &[AtomicU64] {
        let at = (index & self.mask) as usize * W;
        &self.slots[at..at + W]
    }
}

pub struct Producer<'r, 'a, const W: usize> {
    pub(crate) ring: &'r Ring<'a, W>,
    head: u32,
    published: u32,
}

impl<const W: usize> Producer<'_, '_, W> {
    /// Writes one record into the next free slot; not visible until `publish`.
    pub fn push(&mut self, record: &[u64; W]) -> Result<(), RingError> {
        let tail = self.ring.header.tail.load(Ordering::Acquire) & INDEX;
        let used = self.head.wrapping_sub(tail) & INDEX;
        if used > self.ring.capacity() {
            return Err(RingError::Corrupt);
        }
        if used == self.ring.capacity() {
            return Err(RingError::Full);
        }
        for (slot, word) in self.ring.slot(self.head).iter().zip(record.iter()) {
            slot.store(*word, Ordering::Relaxed);
        }
        self.head = self.head.wrapping_add(1) & INDEX;
        Ok(())
    }

    /*
     * Makes every pushed record visible to the consumer, in one swap that
     * also clears the doorbell arm. Returns whether the consumer had armed
     * it, in which case the caller must wake it (`publish_and_ring`).
     */
    #[must_use = "a consumer that armed the doorbell sleeps until it is rung"]
    pub fn publish(&mut self) -> bool {
        if self.head == self.published {
            return false;
        }
        self.published = self.head;
        self.ring.header.head.swap(self.head, Ordering::AcqRel) & ARMED != 0
    }

    /// Records pushed and not yet published.
    pub fn pending(&self) -> u32 {
        self.head.wrapping_sub(self.published)
    }
}

pub struct Consumer<'r, 'a, const W: usize> {
    pub(crate) ring: &'r Ring<'a, W>,
    pub(crate) tail: u32,
}

impl<const W: usize> Consumer<'_, '_, W> {
    /// Published records not yet read, or `Corrupt`.
    pub fn available(&self) -> Result<u32, RingError> {
        self.count(self.ring.header.head.load(Ordering::Acquire))
    }

    /// Records between `tail` and the index in a head word, or `Corrupt`.
    pub(crate) fn count(&self, head: u32) -> Result<u32, RingError> {
        let n = (head & INDEX).wrapping_sub(self.tail) & INDEX;
        if n > self.ring.capacity() {
            return Err(RingError::Corrupt);
        }
        Ok(n)
    }

    /// Reads every published record in order, then releases them all with
    /// one store. Returns how many were read.
    pub fn drain(&mut self, mut each: impl FnMut([u64; W])) -> Result<u32, RingError> {
        let n = self.available()?;
        for i in 0..n {
            let mut record = [0u64; W];
            for (word, slot) in record.iter_mut().zip(self.ring.slot(self.tail.wrapping_add(i))) {
                *word = slot.load(Ordering::Relaxed);
            }
            each(record);
        }
        if n > 0 {
            self.tail = self.tail.wrapping_add(n) & INDEX;
            self.ring.header.tail.store(self.tail, Ordering::Release);
        }
        Ok(n)
    }
}
