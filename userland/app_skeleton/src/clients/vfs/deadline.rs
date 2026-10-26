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

//! How long each vfs call this process makes may wait for its answer.
//!
//! No crate paths here, so host proofs mount this file as it is.

use core::sync::atomic::{AtomicU64, Ordering};

/// What a vfs call that got no answer in time fails with. It is the same
/// words as any call vfs did not answer, which callers already show as "the
/// file store did not answer"; a call that times out never half succeeds.
pub const TIMEOUT: &str = "vfs ipc failed";

static DEADLINE_MS: AtomicU64 = AtomicU64::new(0);

/// Bound every vfs call this process makes, from now on, to `ms`. It is a
/// setting of the whole process, not of one call. 0, the default, leaves
/// each call to the kernel's own reply timeout, as before. A call with no
/// answer by then fails with `Err(TIMEOUT)`; one made of several requests,
/// such as a read or a write of a whole file, stops at the first that
/// times out and fails whole, never returning part of its work as done.
pub fn set_deadline_ms(ms: u64) {
    DEADLINE_MS.store(ms, Ordering::Relaxed);
}

/// The timeout a call that asked for `asked` waits: `asked` when the caller
/// named one, else this process's deadline (0: the kernel's own).
pub fn effective(asked: u64) -> u64 {
    if asked != 0 {
        asked
    } else {
        DEADLINE_MS.load(Ordering::Relaxed)
    }
}
