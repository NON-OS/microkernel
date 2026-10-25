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

//! The contended half of `lock_responsive`: spin, answer shootdowns, and say
//! so once if the wait runs long.

use core::panic::Location;
use spin::{Mutex, MutexGuard};

use super::spin_report::report;
use super::spin_watch::due;

#[cold]
pub(super) fn wait<'a, T>(
    lock: &'a Mutex<T>,
    addr: usize,
    site: &'static Location<'static>,
    sp: usize,
) -> MutexGuard<'a, T> {
    let start = crate::time::timestamp_millis();
    let mut reported = false;
    loop {
        super::serve::serve_shootdowns();
        core::hint::spin_loop();
        if let Some(guard) = lock.try_lock() {
            return guard;
        }
        let now = crate::time::timestamp_millis();
        if due(start, now, reported) {
            reported = true;
            report(addr, site, sp, now - start);
        }
    }
}
