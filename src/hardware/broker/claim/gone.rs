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

//! A claim's entry going (L75, L77). The boot framebuffer's hold is cleared
//! under the table's lock, as `admit` sets it, so a newer claim's hold is
//! never cleared by this one's going; its line is said once the lock is let
//! go, since a serial line takes milliseconds the shutdown's quiesce lacks.

extern crate alloc;

use alloc::vec::Vec;
use spin::MutexGuard;

use super::types::Claim;

/// The function's wipe is owed in the same hold (L78).
pub(super) fn remove(mut claims: MutexGuard<'static, Vec<Claim>>, at: usize) {
    let gone = claims.remove(at);
    super::wipe::owe(gone.device_id, gone.epoch);
    let fb = crate::hardware::broker::bootfb::released(gone.device_id);
    drop(claims);
    if fb {
        crate::hardware::broker::bootfb::say(false);
    }
}
