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

use super::super::types::AllocatorStats;
use super::allocator::PageAllocator;
use spin::Mutex;

pub(super) static PAGE_ALLOCATOR: Mutex<PageAllocator> = Mutex::new(PageAllocator::new());

/// Run `f` on the allocator with interrupts masked, the only way it is taken.
///
/// Masked, because the timer tick frees kernel stacks through this lock, and
/// a tick landing on a holder would spin on its own cpu. Responsive, because
/// that tick spins with interrupts masked, so waiting deaf would leave a
/// shootdown on this cpu unanswered for as long as the holder takes. The
/// holder never waits on a shootdown itself: every critical section here is
/// bookkeeping, with the mapping done outside (see `alloc`).
pub(super) fn with_allocator<R>(f: impl FnOnce(&mut PageAllocator) -> R) -> R {
    crate::arch::run_without_interrupts(|| f(&mut crate::smp::lock_responsive(&PAGE_ALLOCATOR)))
}
pub(super) static ALLOCATOR_STATS: AllocatorStats = AllocatorStats::new();

pub(super) fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}
