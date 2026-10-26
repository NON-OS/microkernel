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

//! Walking away from an exit that stays silent past its budget.

use core::sync::atomic::{AtomicU8, Ordering};

use super::chosen::{EXIT, INDEX};
use super::record::{RECORD, WATCH};

/// Exits walked away from this session for silence, saturating.
static ROTATIONS: AtomicU8 = AtomicU8::new(0);

/// Walk to the next exit if the current one has used up its silence budget.
///
/// Returns whether a rotation happened. The caller owns the consequences:
/// the session bound to the old exit is dead weight and has to be reopened,
/// and connections opened through it will never be answered.
pub fn rotate_if_silent() -> bool {
    let mut watch = WATCH.lock();
    if !watch.should_rotate(nonos_libc::mk_uptime_ms()) {
        return false;
    }
    watch.on_rotate();
    drop(watch);
    if let Some(silent) = EXIT.lock().take() {
        RECORD.lock().silent(&silent);
    }
    INDEX.fetch_add(1, Ordering::AcqRel);
    let _ = ROTATIONS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_add(1));
    true
}

/// Exits walked away from this session for silence.
pub fn rotations() -> u8 {
    ROTATIONS.load(Ordering::Acquire)
}

/// An exit went silent this session, and the one now in use has not
/// delivered yet: the proxy is trying another.
pub fn trying_another() -> bool {
    rotations() > 0 && !WATCH.lock().proven
}
