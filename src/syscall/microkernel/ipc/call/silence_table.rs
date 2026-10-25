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

//! The kernel's one silence table (silence.rs), under a spin lock: a few
//! word compares on a late call or an answer, nothing allocated.

use super::silence::{Pair, Silence};
use super::verdict::DEFAULT_MS;
use crate::sys::diag::Site;
use crate::syscall::microkernel::errnos::ERRNO_TIMEDOUT;
use spin::Mutex;

static TABLE: Mutex<Silence> = Mutex::new(Silence::new(DEFAULT_MS));

/// After a late call of `pair` to `name`: unanswered on `site` when it ends
/// a silence of the default's length, and any pair it evicted, by port.
pub(super) fn report(site: &Site, pair: Pair, name: &str, waited_ms: u64) {
    let now = crate::time::timestamp_millis();
    let [this, evicted] = TABLE.lock().late(pair, now.saturating_sub(waited_ms), now);
    if this.is_some() {
        site.starved(name, ERRNO_TIMEDOUT);
    }
    if let Some(other) = evicted {
        site.silent(other.port);
    }
}

/// An answer in time ends the pair's silence.
pub(super) fn answered(pair: Pair) {
    TABLE.lock().answered(pair);
}
