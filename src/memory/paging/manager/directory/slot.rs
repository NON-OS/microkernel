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

//! The directory's slots, each read whole or read again.
//!
//! A slot is a sequence lock with one writer, the paging manager lock's
//! holder. The writer makes the count odd, writes, and makes it even again;
//! a reader keeps what it read only when the count was even and the same on
//! both sides, so it never pairs one address space's root with another's pid.

use core::sync::atomic::{fence, AtomicBool, AtomicU32, AtomicU64, Ordering::*};

/// One address space as the directory holds it: (asid, owner pid, root).
pub(crate) type Entry = (u32, u32, u64);

pub(crate) struct Slot {
    seq: AtomicU32,
    live: AtomicBool,
    asid: AtomicU32,
    pid: AtomicU32,
    root: AtomicU64,
}

impl Slot {
    pub(crate) const fn empty() -> Self {
        Self {
            seq: AtomicU32::new(0),
            live: AtomicBool::new(false),
            asid: AtomicU32::new(0),
            pid: AtomicU32::new(0),
            root: AtomicU64::new(0),
        }
    }

    /// Fill the slot with `entry`, or empty it. Writer only.
    pub(crate) fn write(&self, entry: Option<Entry>) {
        self.seq.fetch_add(1, Relaxed);
        fence(Release);
        let (asid, pid, root) = entry.unwrap_or_default();
        self.asid.store(asid, Relaxed);
        self.pid.store(pid, Relaxed);
        self.root.store(root, Relaxed);
        self.live.store(entry.is_some(), Relaxed);
        self.seq.fetch_add(1, Release);
    }

    /// The slot as one whole write left it, None when it is empty.
    pub(crate) fn read(&self) -> Option<Entry> {
        loop {
            let before = self.seq.load(Acquire);
            let live = self.live.load(Relaxed);
            let entry = (self.asid.load(Relaxed), self.pid.load(Relaxed), self.root.load(Relaxed));
            fence(Acquire);
            if before & 1 == 0 && self.seq.load(Relaxed) == before {
                return live.then_some(entry);
            }
            core::hint::spin_loop();
        }
    }
}
