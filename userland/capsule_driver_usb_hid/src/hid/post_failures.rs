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

//! Events the kernel input ring refused, counted for the life of the driver
//! and reported by GET_STATE: key events apart from pointer events. The
//! counts are the driver's, not a device's, so they outlive an unplug.

use core::sync::atomic::{AtomicU64, Ordering};

static KEYS: AtomicU64 = AtomicU64::new(0);
static POINTER: AtomicU64 = AtomicU64::new(0);

/// Count a key post that did not go through (`posted` false).
pub fn count_key(posted: bool) {
    if !posted {
        KEYS.fetch_add(1, Ordering::Relaxed);
    }
}

/// Count a pointer post that did not go through (`posted` false).
pub fn count_pointer(posted: bool) {
    if !posted {
        POINTER.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn key_post_failures() -> u64 {
    KEYS.load(Ordering::Relaxed)
}

pub fn pointer_post_failures() -> u64 {
    POINTER.load(Ordering::Relaxed)
}
