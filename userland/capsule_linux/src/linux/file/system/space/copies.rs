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
 * The copies a family is writing, held to what the store would take of
 * them: its private directories' bytes and its home's. They were held to
 * the private directories' alone, 16 MiB, and refused without a word: a
 * synced Monero wallet's save, a 21 MB cache in its 64 MiB home, failed
 * with only "failed to save file". Now a refusal is said once a run.
 */

use core::sync::atomic::{AtomicBool, Ordering};

use super::super::declared::PRIVATE;
use super::home_size::home_bytes;
use super::quota;

static TOLD: AtomicBool = AtomicBool::new(false);

/// Whether one copy may go from `was` bytes to `len` beside `total`.
pub fn copies_allow(total: u64, was: u64, len: u64) -> Result<(), i64> {
    let most = PRIVATE.saturating_add(home_bytes());
    let refused = quota::copies_allow(total, was, len, most);
    if refused.is_err() && !TOLD.swap(true, Ordering::Relaxed) {
        let line = alloc::format!(
            "linux: this app is writing more at once than it may hold here ({} MiB \
             across the files it has open); that write was refused.\n",
            most >> 20
        );
        crate::linux::say::say(line.as_bytes());
    }
    refused
}
