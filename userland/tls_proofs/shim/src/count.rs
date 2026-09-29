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

//! Calls made on this thread. Tests run on threads of their own, so each
//! reads only its own.

use std::cell::Cell;

/// How many of each call reached the pool, or the kernel, on this thread.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    /// Syscalls the kernel forwards to the pool and waits on.
    pub kernel_round_trips: u32,
    pub x25519: u32,
    /// Direct calls to the pool, by operation.
    pub rsa: u32,
    pub p256: u32,
    pub p384: u32,
    pub sha384: u32,
    pub other_ipc: u32,
    pub lookups: u32,
}

thread_local! {
    static COUNTS: Cell<Counts> = Cell::new(Counts::default());
}

pub fn counts() -> Counts {
    COUNTS.with(|c| c.get())
}

pub fn reset() {
    COUNTS.with(|c| c.set(Counts::default()));
}

pub(crate) fn note(f: impl FnOnce(&mut Counts)) {
    COUNTS.with(|c| {
        let mut now = c.get();
        f(&mut now);
        c.set(now);
    });
}
