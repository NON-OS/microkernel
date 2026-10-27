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

//! What one install spent its time on (fast downloads, A): the index's
//! GETs and the packages' GETs, apart, with their bytes; verifying; placing
//! in the store. Begun with the install, said once when it ends.

mod end;
mod line;
mod spend;

pub use end::timed;
pub use spend::{placing, verifying};

use core::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// The counts, by these slots: GETs, bytes and milliseconds of each phase
/// (index, then packages), the phase, verifying, placing, and the start.
static N: [AtomicU64; 10] = [const { AtomicU64::new(0) }; 10];
const GETS: usize = 0;
const BYTES: usize = 2;
const NET_MS: usize = 4;
const PHASE: usize = 6;
const VERIFY_MS: usize = 7;
const PLACE_MS: usize = 8;
const START: usize = 9;

pub fn now() -> u64 {
    u64::try_from(nonos_libc::mk_time_millis()).unwrap_or(0)
}

/// The index is in; what follows is packages.
pub fn packages() {
    N[PHASE].store(1, Relaxed);
}

/// One GET: what came and how long it took.
pub fn got(bytes: usize, ms: u64) {
    let at = N[PHASE].load(Relaxed).min(1) as usize;
    N[GETS + at].fetch_add(1, Relaxed);
    N[BYTES + at].fetch_add(bytes as u64, Relaxed);
    N[NET_MS + at].fetch_add(ms, Relaxed);
}
