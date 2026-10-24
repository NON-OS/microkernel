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

//! Where the device tree put the TPM's register window, read once at boot.
//! Zero length for a board whose tree names none.

use core::sync::atomic::{AtomicU64, Ordering};

static BASE: AtomicU64 = AtomicU64::new(0);
static LEN: AtomicU64 = AtomicU64::new(0);

/// Recorded during the device-tree parse, before any CPU but the boot one
/// runs, and never changed after.
pub fn set(base: u64, len: u64) {
    BASE.store(base, Ordering::Relaxed);
    LEN.store(len, Ordering::Release);
}

pub fn get() -> Option<(u64, u64)> {
    match LEN.load(Ordering::Acquire) {
        0 => None,
        len => Some((BASE.load(Ordering::Relaxed), len)),
    }
}
