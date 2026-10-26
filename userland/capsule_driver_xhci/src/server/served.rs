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
//! What the server knows of the one request it is serving: who sent it,
//! whether the multiplexer vouched for its slot, and whether a reply is
//! being held for the multiplexer to rewrite (reply.rs, vouch.rs). The
//! capsule serves one request at a time on its one thread, so on the
//! target this is one static. The host proofs run the same server on many
//! test threads at once, each a server of its own, so under test each
//! thread holds its own: shared, one test's capture swallowed another's
//! reply and its release sent a second one.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize};

pub(super) struct Served {
    pub(super) sender: AtomicU32,
    pub(super) vouched: AtomicBool,
    pub(super) capture: AtomicBool,
    pub(super) captured_len: AtomicUsize,
}

impl Served {
    const fn new() -> Self {
        Self {
            sender: AtomicU32::new(0),
            vouched: AtomicBool::new(false),
            capture: AtomicBool::new(false),
            captured_len: AtomicUsize::new(0),
        }
    }
}

#[cfg(not(test))]
static SERVED: Served = Served::new();

/// Run `f` on the request being served.
#[cfg(not(test))]
pub(super) fn served<R>(f: impl FnOnce(&Served) -> R) -> R {
    f(&SERVED)
}

#[cfg(test)]
std::thread_local! {
    static SERVED: Served = const { Served::new() };
}

#[cfg(test)]
pub(super) fn served<R>(f: impl FnOnce(&Served) -> R) -> R {
    SERVED.with(f)
}
