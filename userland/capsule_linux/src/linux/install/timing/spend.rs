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

//! A stage of an install timed as it runs: what it took is added to its
//! count (timing/mod.rs).

use core::sync::atomic::Ordering::Relaxed;

use super::{now, N, PLACE_MS, VERIFY_MS};

/// `work`, its time added to `slot`'s.
fn spend<T>(slot: usize, work: impl FnOnce() -> T) -> T {
    let t0 = now();
    let done = work();
    N[slot].fetch_add(now().saturating_sub(t0), Relaxed);
    done
}

pub fn verifying<T>(work: impl FnOnce() -> T) -> T {
    spend(VERIFY_MS, work)
}

pub fn placing<T>(work: impl FnOnce() -> T) -> T {
    spend(PLACE_MS, work)
}
