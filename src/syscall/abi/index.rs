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

//! The registry indexed: a syscall's slot is its position in registry
//! order, and `TABLE`, a hash table over the tags, finds it in a probe or
//! two. Both are built at compile time from `REGISTRY`, so they cannot
//! drift from it.

use super::registry::REGISTRY;
use super::AbiEntry;

const fn count() -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < REGISTRY.len() {
        n += REGISTRY[i].len();
        i += 1;
    }
    n
}

/// Entries in the registry, and so the number of slots.
pub const COUNT: usize = count();

const fn in_order() -> [&'static AbiEntry; COUNT] {
    let mut out = [&REGISTRY[0][0]; COUNT];
    let mut n = 0;
    let mut i = 0;
    while i < REGISTRY.len() {
        let mut j = 0;
        while j < REGISTRY[i].len() {
            out[n] = &REGISTRY[i][j];
            n += 1;
            j += 1;
        }
        i += 1;
    }
    out
}

/*
 * Open addressing over a power-of-two table at most a quarter full, so every
 * probe sequence ends at an empty bucket. The multiplier spreads the
 * four ASCII bytes of a tag over the top bits the bucket is taken from.
 */
const BUCKET_BITS: u32 = 10;
const BUCKETS: usize = 1 << BUCKET_BITS;
const EMPTY: u16 = u16::MAX;
const _: () = assert!(COUNT * 4 <= BUCKETS && COUNT < EMPTY as usize);

const fn bucket(id: u64) -> usize {
    (id.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> (64 - BUCKET_BITS)) as usize
}

const fn table() -> [u16; BUCKETS] {
    let mut out = [EMPTY; BUCKETS];
    let mut n = 0;
    while n < COUNT {
        let mut b = bucket(SLOTS[n].id);
        while out[b] != EMPTY {
            b = (b + 1) & (BUCKETS - 1);
        }
        out[b] = n as u16;
        n += 1;
    }
    out
}

static SLOTS: [&AbiEntry; COUNT] = in_order();
static TABLE: [u16; BUCKETS] = table();

/// The slot of the syscall tagged `id`, if there is one.
#[inline]
pub fn slot_of(id: u64) -> Option<usize> {
    let mut b = bucket(id);
    loop {
        let slot = TABLE[b];
        if slot == EMPTY {
            return None;
        }
        if SLOTS[usize::from(slot)].id == id {
            return Some(usize::from(slot));
        }
        b = (b + 1) & (BUCKETS - 1);
    }
}

/// The longest probe sequence a registered tag takes; kept short by test.
pub fn longest_probe() -> usize {
    let mut worst = 0;
    for (n, e) in SLOTS.iter().enumerate() {
        let mut b = bucket(e.id);
        let mut probes = 1;
        while usize::from(TABLE[b]) != n {
            b = (b + 1) & (BUCKETS - 1);
            probes += 1;
        }
        worst = worst.max(probes);
    }
    worst
}

/// The registry entry in `slot`.
#[inline]
pub fn entry(slot: usize) -> Option<&'static AbiEntry> {
    SLOTS.get(slot).copied()
}
