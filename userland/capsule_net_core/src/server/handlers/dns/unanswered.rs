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

//! Says on the log that a lookup ran out of time with no DNS server
//! answering, the one failure a photo cannot otherwise tell from a name that
//! does not exist.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::mk_debug;

static SAID_AT: AtomicI64 = AtomicI64::new(i64::MIN);

/// Once every ten seconds at most: a page asks for many names at once.
pub(super) fn say_unanswered(now: i64) {
    if now.saturating_sub(SAID_AT.load(Ordering::Relaxed)) >= 10_000 {
        SAID_AT.store(now, Ordering::Relaxed);
        let line = b"[NET-CORE] dns: no server the lease named answered in 3 s\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
}
