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

//! The invalidation queue each unit runs, kept beside the unit list. A queue
//! is live once its unit acknowledged QIE; from then on every invalidation on
//! that unit goes through it, because the spec leaves register-based
//! invalidation undefined while the queue is enabled (VT-d 3.4, 6.5.2).

use spin::Mutex;

use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::probe::MAX_UNITS;
use crate::arch::x86_64::iommu::unit::report::units;

pub(super) struct Queue {
    /// The descriptor page, or zero while the unit has no queue.
    pub ring_phys: u64,
    /// The page whose first dword a wait descriptor writes.
    pub status_phys: u64,
    /// Next slot software fills, as last written to IQT.
    pub tail: u16,
    /// The value the next wait asks for, so a late answer to an earlier
    /// wait is never taken for this one.
    pub sequence: u32,
    pub live: bool,
}

const IDLE: Mutex<Queue> =
    Mutex::new(Queue { ring_phys: 0, status_phys: 0, tail: 0, sequence: 0, live: false });

pub(super) static QUEUES: [Mutex<Queue>; MAX_UNITS] = [IDLE; MAX_UNITS];

/// The queue slot of a probed unit: its position in DMAR order.
pub(super) fn slot_of(unit: &RemapUnit) -> Option<&'static Mutex<Queue>> {
    let index = units().iter().position(|u| u.unit.base_pa() == unit.base_pa())?;
    QUEUES.get(index)
}

/// Whether invalidations on `unit` go through its queue.
pub fn is_live(unit: &RemapUnit) -> bool {
    slot_of(unit).is_some_and(|q| q.lock().live)
}
