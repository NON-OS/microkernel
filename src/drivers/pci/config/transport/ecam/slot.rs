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

//! A published window as two words, so a reader loads it without a lock.

use core::sync::atomic::{AtomicU64, Ordering};

use super::window::EcamWindow;

pub(super) struct Slot {
    key: AtomicU64,
    va: AtomicU64,
}

impl Slot {
    pub(super) const fn new() -> Self {
        Self { key: AtomicU64::new(0), va: AtomicU64::new(0) }
    }

    /// Only the registry's writer calls this, on a slot no reader can see
    /// yet; the count's release store publishes it.
    pub(super) fn store(&self, w: EcamWindow) {
        self.key.store(pack(w), Ordering::Relaxed);
        self.va.store(w.va, Ordering::Relaxed);
    }

    /// Only for a slot below a count read with acquire.
    pub(super) fn load(&self) -> EcamWindow {
        let key = self.key.load(Ordering::Relaxed);
        EcamWindow {
            segment: (key >> 16) as u16,
            start_bus: (key >> 8) as u8,
            end_bus: key as u8,
            va: self.va.load(Ordering::Relaxed),
        }
    }
}

const fn pack(w: EcamWindow) -> u64 {
    ((w.segment as u64) << 16) | ((w.start_bus as u64) << 8) | w.end_bus as u64
}
