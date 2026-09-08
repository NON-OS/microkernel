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

//! Whether the store has changed since the last time anyone looked.
//!
//! The desktop icons and the package list both need to notice a new file, and
//! both used to find out by listing a directory on every clock tick and
//! comparing the result. That is a tree walk, a serialisation and an IPC copy
//! once a second for the life of the session, to learn nothing almost every
//! time.
//!
//! One counter read answers for both. It is deliberately shared rather than one
//! per caller: they run in the same pass, so a second read would return the
//! same number and cost another round trip to learn it.

use core::sync::atomic::{AtomicU64, Ordering};

/// Last generation acted on. Starts at a value the store cannot report, so the
/// first pass always lists: a desktop that waits for a change before drawing
/// anything would come up empty.
const NEVER: u64 = u64::MAX;
static SEEN: AtomicU64 = AtomicU64::new(NEVER);

/// True when the store may have changed, or when vfs_pool will not say.
///
/// A missing answer counts as changed. vfs_pool is unreachable during package
/// staging, and a shell that treated silence as "nothing new" would hold a
/// stale desktop for as long as the service stayed quiet.
pub fn since_last_look() -> bool {
    let Some(now) = crate::vfs_client::generation() else {
        return true;
    };
    SEEN.swap(now, Ordering::Relaxed) != now
}
