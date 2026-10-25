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

//! Taking a lock the responsive way, whatever kind it is: try once, and when
//! that fails spin, answer shootdowns, and say so once if the wait runs long.
//! Every take is noted, so a later waiter can name its holder.

use core::panic::Location;

use super::spin_holders::note;
use super::spin_report::report;
use super::spin_watch::due;

/// Take the lock at `addr` through `try_take`, for the caller at `site`
/// whose frame is at `sp`; `shared` when it is a reader's take.
#[inline]
pub(super) fn take<G>(
    addr: usize,
    site: &'static Location<'static>,
    sp: usize,
    shared: bool,
    mut try_take: impl FnMut() -> Option<G>,
) -> G {
    let guard = match try_take() {
        Some(guard) => guard,
        None => wait(addr, site, sp, try_take),
    };
    note(addr, site, sp, shared);
    guard
}

#[cold]
fn wait<G>(
    addr: usize,
    site: &'static Location<'static>,
    sp: usize,
    mut try_take: impl FnMut() -> Option<G>,
) -> G {
    let start = crate::time::timestamp_millis();
    let mut reported = false;
    loop {
        super::serve::serve_shootdowns();
        core::hint::spin_loop();
        if let Some(guard) = try_take() {
            return guard;
        }
        let now = crate::time::timestamp_millis();
        if due(start, now, reported) {
            reported = true;
            report(addr, site, sp, now - start);
        }
    }
}
