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

//! A kept frame a driver can hold in a `static`, for a serve path that owns
//! no state of its own between requests (nonos_wifi_core's netif). A driver
//! serves one request at a time, so the cell is never contended; if it were,
//! the request gets a fresh frame and nothing kept, the old wire's answer.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::resend::Kept;

pub struct KeptCell<const N: usize> {
    busy: AtomicBool,
    kept: UnsafeCell<Kept<N>>,
}

// SAFETY: `kept` is reached only inside `with`, after `busy` went from false
// to true by this caller, and `busy` is set false again only after the
// closure's borrow has ended; so at most one `&mut Kept` exists at a time.
unsafe impl<const N: usize> Sync for KeptCell<N> {}

impl<const N: usize> KeptCell<N> {
    pub const fn new() -> Self {
        KeptCell { busy: AtomicBool::new(false), kept: UnsafeCell::new(Kept::new()) }
    }

    /// Run `f` on the kept frame, or `None` when the cell is in use.
    pub fn with<T>(&self, f: impl FnOnce(&mut Kept<N>) -> T) -> Option<T> {
        self.busy.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).ok()?;
        // SAFETY: `busy` is held by this call (see the Sync impl above).
        let out = f(unsafe { &mut *self.kept.get() });
        self.busy.store(false, Ordering::Release);
        Some(out)
    }
}

impl<const N: usize> Default for KeptCell<N> {
    fn default() -> Self {
        Self::new()
    }
}
