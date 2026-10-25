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

//! Waits on the uptime clock. The first looks are a yield apart, for the
//! answer a host gives at once; after them the wait sleeps a millisecond
//! between looks, so a slow host costs no spinning core.

use nonos_libc::{mk_idle_ms, mk_yield, Deadline};

use crate::constants::QUICK_LOOKS;

/// The first `Some` `look` gives within `ms` milliseconds. It is asked
/// once more at the deadline, so an answer during the last sleep counts.
pub fn until<T>(ms: u64, mut look: impl FnMut() -> Option<T>) -> Option<T> {
    let deadline = Deadline::after_ms(ms);
    let mut looks = 0u32;
    loop {
        if let Some(v) = look() {
            return Some(v);
        }
        if deadline.expired() {
            return look();
        }
        if looks < QUICK_LOOKS {
            looks += 1;
            let _ = mk_yield();
        } else {
            let _ = mk_idle_ms(1);
        }
    }
}
