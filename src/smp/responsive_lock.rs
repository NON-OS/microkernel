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

//! The locks `lock_responsive` takes: a plain spin mutex, which goes to
//! whichever waiter tries first, and a ticket mutex, which goes in turn.

use core::panic::Location;
use spin::{Mutex, MutexGuard};

use super::spin_wait::take;
use super::ticket::{TicketGuard, TicketMutex};

/// A lock taken the responsive way (responsive.rs).
pub trait Responsive {
    type Guard<'a>
    where
        Self: 'a;

    /// Take it for the caller at `site`, whose frame is at `sp`.
    fn take_at(&self, site: &'static Location<'static>, sp: usize) -> Self::Guard<'_>;
}

impl<T> Responsive for Mutex<T> {
    type Guard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;

    fn take_at(&self, site: &'static Location<'static>, sp: usize) -> MutexGuard<'_, T> {
        take(self as *const Self as usize, site, sp, false, || self.try_lock())
    }
}

impl<T> Responsive for TicketMutex<T> {
    type Guard<'a>
        = TicketGuard<'a, T>
    where
        Self: 'a;

    /// The number drawn here is waited for whatever happens, so the line
    /// behind it never stops.
    fn take_at(&self, site: &'static Location<'static>, sp: usize) -> TicketGuard<'_, T> {
        let ticket = self.draw();
        take(self as *const Self as usize, site, sp, false, || self.served(ticket))
    }
}
