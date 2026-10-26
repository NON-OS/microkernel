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

//! What a trip lock grants (L81b): a read, or the write when `WHOLE`,
//! given back on drop, waking who waits once the lock is free.

use core::ops::{Deref, DerefMut};

use super::trip_lock::TripLock;

pub struct TripGuard<'a, T, const WHOLE: bool> {
    lock: &'a TripLock<T>,
    me: u32,
}

pub type TripRead<'a, T> = TripGuard<'a, T, false>;
pub type TripWrite<'a, T> = TripGuard<'a, T, true>;

impl<T> TripLock<T> {
    pub fn read(&self) -> TripRead<'_, T> {
        TripGuard { lock: self, me: self.wait(false) }
    }

    pub fn write(&self) -> TripWrite<'_, T> {
        TripGuard { lock: self, me: self.wait(true) }
    }
}

impl<T, const WHOLE: bool> Deref for TripGuard<'_, T, WHOLE> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: held, so no writer but this guard's own is inside.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for TripGuard<'_, T, true> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the write is held, so no one else is inside.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T, const WHOLE: bool> Drop for TripGuard<'_, T, WHOLE> {
    fn drop(&mut self) {
        if WHOLE {
            self.lock.left(false);
        }
        let (had, woken) = self.lock.rows.lock().give(self.me, WHOLE);
        super::trip_back::gave(self.me, had);
        woken.into_iter().for_each(crate::sched::wake_process);
    }
}
