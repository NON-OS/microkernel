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

//! Requester ids noted without a lock, pure: the fault poll notes them
//! in the timer interrupt, and the broker takes them where locks may be
//! taken. kernel_proofs::iommu_attach.

use core::sync::atomic::{AtomicU32, Ordering};

/// Up to `N` distinct ids, each held plus one so zero is a free slot.
pub(crate) struct Noted<const N: usize>([AtomicU32; N]);

impl<const N: usize> Noted<N> {
    pub(crate) const fn new() -> Self {
        Self([const { AtomicU32::new(0) }; N])
    }

    /// False when every slot holds another id: the next fault notes it again.
    pub(crate) fn note(&self, source: u16) -> bool {
        let value = u32::from(source) + 1;
        if self.0.iter().any(|slot| slot.load(Ordering::Acquire) == value) {
            return true;
        }
        let free = |slot: &AtomicU32| {
            slot.compare_exchange(0, value, Ordering::AcqRel, Ordering::Acquire).is_ok()
        };
        self.0.iter().any(free)
    }

    /// Every id noted, each once, the slots freed as they are read.
    pub(crate) fn take(&self) -> impl Iterator<Item = u16> + '_ {
        self.0.iter().filter_map(|slot| match slot.swap(0, Ordering::AcqRel) {
            0 => None,
            value => Some((value - 1) as u16),
        })
    }
}
