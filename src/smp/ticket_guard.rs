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

//! What holding a `TicketMutex` gives: its data, and the next number served
//! when the hold ends.

use core::ops::{Deref, DerefMut};

use super::ticket::TicketMutex;

pub struct TicketGuard<'a, T> {
    lock: &'a TicketMutex<T>,
}

impl<'a, T> TicketGuard<'a, T> {
    /// Only `TicketMutex` makes one, for the number it is serving.
    pub(super) fn new(lock: &'a TicketMutex<T>) -> Self {
        Self { lock }
    }

    fn lock(&self) -> &TicketMutex<T> {
        self.lock
    }
}

impl<T> Deref for TicketGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: this guard is the one hold the lock's number allows.
        unsafe { &*self.lock().data() }
    }
}

impl<T> DerefMut for TicketGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: as for `deref`, and the guard is borrowed mutably.
        unsafe { &mut *self.lock().data() }
    }
}

impl<T> Drop for TicketGuard<'_, T> {
    fn drop(&mut self) {
        // Only the holder moves the count, so each number is served once.
        self.lock().serve_next();
    }
}
