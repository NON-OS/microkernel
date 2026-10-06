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

//! A count of the writes made to the volume, for anything that keeps what
//! it read: whatever it kept under an older count may be stale.
//!
//! Every sealed write advances it after its sector is held for the device,
//! and so does a new window. A reader takes the count before it reads
//! anything; if the count has moved when it next looks, a write may have
//! come in between and what it kept is thrown away. A reader that took the
//! count after a write finds that write's sector held, and every fetch
//! sends what is held before it reads, so it cannot see the sector's older
//! contents either.

use core::sync::atomic::{AtomicU64, Ordering};

static EPOCH: AtomicU64 = AtomicU64::new(0);

/// The current count.
pub fn epoch() -> u64 {
    EPOCH.load(Ordering::SeqCst)
}

/// A write was made, or the volume moved.
pub(super) fn advance() {
    EPOCH.fetch_add(1, Ordering::SeqCst);
}
