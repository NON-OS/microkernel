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

//! How long the calls to driver.xhci0 may still take. A block request and a
//! probe step each open a window, and every call in it waits at most what
//! is left of it, so neither holds the server past its bound however many
//! calls it makes. A call with nothing left is not sent, and fails
//! `E_TIMEDOUT`. A window opened inside another replaces it until it
//! closes: a reset after a command that ran out of time gets time of its
//! own, so the timeout still ends in a reset.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::clock::now_ms;

/// No window open: each call waits its own whole timeout.
const NONE: u64 = u64::MAX;

/// The uptime the open window closes at. One thread runs this driver.
static END_MS: AtomicU64 = AtomicU64::new(NONE);

/// An open window; the one it was opened in is back when it drops.
pub struct Window {
    outer: u64,
}

impl Window {
    pub fn open(ms: u64) -> Self {
        let end = now_ms().map_or(0, |t| t.saturating_add(ms).min(NONE - 1));
        Self { outer: END_MS.swap(end, Ordering::Relaxed) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        END_MS.store(self.outer, Ordering::Relaxed);
    }
}

/// What the next call may wait, at most `cap`; nothing once the window is
/// spent or the clock cannot be read.
pub fn left_ms(cap: u64) -> u64 {
    let end = END_MS.load(Ordering::Relaxed);
    if end == NONE {
        return cap;
    }
    now_ms().map_or(0, |now| end.saturating_sub(now)).min(cap)
}
