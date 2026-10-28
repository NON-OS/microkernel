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

/*
 * How much this run's home keeps: declared::HOME, or what the listing the
 * run started for is given (app/home_listings.rs), chosen once, as its home
 * opens. One capsule is one run, so one home.
 */

use core::sync::atomic::{AtomicU64, Ordering};

use super::super::declared::{HOME, HOME_MOST};
use crate::linux::app::LARGER_HOMES;

static BYTES: AtomicU64 = AtomicU64::new(HOME);

// No listing is given more than the store was sized for (declared::HOMES).
const _: () = {
    let mut i = 0;
    while i < LARGER_HOMES.len() {
        assert!(LARGER_HOMES[i].1 > HOME && LARGER_HOMES[i].1 <= HOME_MOST);
        i += 1;
    }
};

/// The home size of the listing whose key is `listing`.
pub fn of(listing: &[u8; 32]) -> u64 {
    LARGER_HOMES.iter().find(|(key, _)| key == listing).map_or(HOME, |&(_, bytes)| bytes)
}

/// Chosen as the home opens, for the listing the run's policy names.
pub fn choose(listing: &[u8; 32]) {
    BYTES.store(of(listing), Ordering::Relaxed);
}

/// This run's home size, in bytes.
pub fn home_bytes() -> u64 {
    BYTES.load(Ordering::Relaxed)
}
