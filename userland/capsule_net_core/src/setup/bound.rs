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

//! The NIC the stack is bound to.

use core::sync::atomic::{AtomicU32, Ordering};

// The broker port of the NIC the stack is currently bound to. A periodic
// re-evaluation compares against this to notice when a better link (the WiFi link
// coming up after boot) should replace the one bound at startup.
pub(super) static BOUND_PORT: AtomicU32 = AtomicU32::new(0);

/// The broker port of the interface the stack is bound to, or zero before the
/// first bind. Read by the lease-status reply so a panel can see which NIC the
/// stack chose when no address ever binds.
pub fn bound_port() -> u32 {
    BOUND_PORT.load(Ordering::Acquire)
}
