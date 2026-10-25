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

//! `lock_responsive` for a reader-writer lock: the same shootdown service while
//! it waits and the same report when a wait runs long (smp-lock-waits.md). A
//! reader waits only while a writer holds the lock, so a long wait means the
//! same thing for both.

use core::panic::Location;
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::spin_holders::stack_mark;
use super::spin_wait::take;

/// Share `lock` for reading, as `lock_responsive` takes a mutex.
#[track_caller]
pub fn read_responsive<T>(lock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    let addr = lock as *const RwLock<T> as usize;
    take(addr, Location::caller(), stack_mark(), true, || lock.try_read())
}

/// Hold `lock` alone for writing, as `lock_responsive` takes a mutex.
#[track_caller]
pub fn write_responsive<T>(lock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    let addr = lock as *const RwLock<T> as usize;
    take(addr, Location::caller(), stack_mark(), false, || lock.try_write())
}
