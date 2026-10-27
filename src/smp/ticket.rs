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

//! A lock handed over in the order it was asked for.
//!
//! A spin::Mutex goes to whichever CPU's try lands first after a release. A
//! CPU that lets go and takes again in a loop, as a mapping loop does page by
//! page, nearly always wins that race against one spinning from further away,
//! and the paging manager's waiters lost it for a second and more at a time
//! (regression-30a4b58c4-paging-lock). Here each taker draws the next number
//! and is served when its number comes up, so no taker waits behind more
//! than the ones that asked first.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU32, Ordering::*};

pub use super::ticket_guard::TicketGuard;

pub struct TicketMutex<T> {
    next: AtomicU32,
    serving: AtomicU32,
    data: UnsafeCell<T>,
}

// SAFETY: the data is reached only through a guard, and only the taker whose
// number is being served holds one.
unsafe impl<T: Send> Sync for TicketMutex<T> {}
unsafe impl<T: Send> Send for TicketMutex<T> {}

impl<T> TicketMutex<T> {
    pub const fn new(data: T) -> Self {
        Self { next: AtomicU32::new(0), serving: AtomicU32::new(0), data: UnsafeCell::new(data) }
    }

    /// The next number in line. It must be waited for (`served`): a number
    /// drawn and never served stops every taker after it.
    pub fn draw(&self) -> u32 {
        self.next.fetch_add(1, Relaxed)
    }

    /// The lock for `ticket`, once that number is being served.
    pub fn served(&self, ticket: u32) -> Option<TicketGuard<'_, T>> {
        (self.serving.load(Acquire) == ticket).then(|| TicketGuard::new(self))
    }

    /// The lock, only when nobody holds it or waits for it.
    pub fn try_lock(&self) -> Option<TicketGuard<'_, T>> {
        let now = self.serving.load(Acquire);
        let taken = self.next.compare_exchange(now, now.wrapping_add(1), Acquire, Relaxed);
        taken.ok().map(|_| TicketGuard::new(self))
    }

    pub(super) fn data(&self) -> *mut T {
        self.data.get()
    }

    /// The hold ends: Release hands the next number what it wrote.
    pub(super) fn serve_next(&self) {
        self.serving.fetch_add(1, Release);
    }
}
