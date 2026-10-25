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

//! How long a NONOS disk that is not the boot disk waits for one that may be.
//!
//! The loader's own disk wins, but a driver still starting cannot say whether
//! its disk is that one. Every disk that answered is the whole answer; while
//! one has not, the other NONOS disk waits, and not past `BOOT_WAIT_MS`: a
//! driver that never comes up must not leave the machine with no disk.

use core::sync::atomic::AtomicU64;

use super::wait_since::{over, stamp};
use crate::sys::timer::uptime_ms;

const BOOT_WAIT_MS: u64 = 15_000;

/// When a disk was first refused, in uptime milliseconds plus one; 0 before.
static SINCE: AtomicU64 = AtomicU64::new(0);

/// Start the wait at the first refusal, not after a whole pass: a pass that
/// meets a silent driver is itself long, and that time counts. A killed
/// caller's refusals are its own end, never the disk's, so they start nothing.
pub(super) fn refused_now() {
    if !caller_gone() {
        stamp(&SINCE, uptime_ms());
    }
}

pub(super) fn waited_for_boot_disk() -> bool {
    refused_now();
    over(&SINCE, uptime_ms(), BOOT_WAIT_MS)
}

/// The caller was killed meanwhile (L81): its asks failed for its own end,
/// so no disk is kept on what they said.
pub(super) fn caller_gone() -> bool {
    crate::process::current_pid().is_some_and(|p| crate::process::scheduler::selection::is_dead(p))
}
