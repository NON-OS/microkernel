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

//! Which process each CPU is running.

use core::sync::atomic::{AtomicU32, Ordering};

pub struct CurrentPid {
    slots: [AtomicU32; crate::smp::MAX_CPUS],
}

impl CurrentPid {
    pub const fn new() -> Self {
        Self { slots: [const { AtomicU32::new(0) }; crate::smp::MAX_CPUS] }
    }

    #[inline]
    fn slot(&self) -> &AtomicU32 {
        &self.slots[crate::smp::cpu_id()]
    }

    #[inline]
    pub fn load(&self, order: Ordering) -> u32 {
        self.slot().load(order)
    }

    #[inline]
    pub fn store(&self, value: u32, order: Ordering) {
        self.slot().store(value, order);
    }

    #[inline]
    pub fn swap(&self, value: u32, order: Ordering) -> u32 {
        self.slot().swap(value, order)
    }
}

pub static CURRENT_PID: CurrentPid = CurrentPid::new();
