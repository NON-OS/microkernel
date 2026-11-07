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

use core::sync::atomic::{AtomicU64, Ordering};

/*
 * The revocation epoch only moves up and never wraps. A wrapped epoch would
 * make a token minted before every revocation compare as current again, so
 * at the top the advance is refused and the caller withdraws everything
 * instead (`reissue`). 2^64 revocations of one process is not reachable in
 * practice; the bound is here so that the comparison never has to assume it.
 */
pub(crate) fn advance(epoch: &AtomicU64) -> Option<u64> {
    let mut seen = epoch.load(Ordering::Acquire);
    loop {
        let next = seen.checked_add(1)?;
        match epoch.compare_exchange_weak(seen, next, Ordering::SeqCst, Ordering::Acquire) {
            Ok(_) => return Some(next),
            Err(now) => seen = now,
        }
    }
}
