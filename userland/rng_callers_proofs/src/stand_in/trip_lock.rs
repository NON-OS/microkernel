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

//! The data volume's trip lock (services/lifecycle/trip_lock.rs), as these
//! tests need it: one thread holds it at a time, so a reader-writer lock,
//! and a writer leaving lets go as the kernel's does. No pid is ever handed
//! on here, so there is no row to forget.

pub mod lifecycle {
    use core::ops::{Deref, DerefMut};
    use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub struct TripLock<T> {
        data: RwLock<T>,
        let_go: fn(&mut T, bool),
    }

    pub struct TripWrite<'a, T> {
        held: RwLockWriteGuard<'a, T>,
        let_go: fn(&mut T, bool),
    }

    impl<T> TripLock<T> {
        pub const fn new(data: T, let_go: fn(&mut T, bool)) -> Self {
            TripLock { data: RwLock::new(data), let_go }
        }

        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            self.data.read()
        }

        pub fn write(&self) -> TripWrite<'_, T> {
            TripWrite { held: self.data.write(), let_go: self.let_go }
        }

        pub fn forget(&self, _pid: u32) {}
    }

    impl<T> Deref for TripWrite<'_, T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.held
        }
    }

    impl<T> DerefMut for TripWrite<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.held
        }
    }

    impl<T> Drop for TripWrite<'_, T> {
        fn drop(&mut self) {
            (self.let_go)(&mut self.held, false);
        }
    }
}
