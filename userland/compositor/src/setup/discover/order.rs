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

//! Which display driver the display log names. The lookup runs every few
//! milliseconds while the compositor waits for a driver and while it is on
//! GOP, so the log gets one line for each driver found anew, not one a try.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::displays::DISPLAYS;

/// The driver to name for a lookup that found the one at `found` in the
/// kernel's list, or `None`: when `said` names it already, and when the
/// lookup found none, as the compositor may still be waiting for one; it is
/// on GOP only once it takes the GOP framebuffer, and wait_for_setup says
/// so then.
pub fn news(said: &AtomicUsize, found: Option<usize>) -> Option<&'static str> {
    let now = found.unwrap_or(DISPLAYS.len());
    if said.swap(now, Ordering::Relaxed) == now {
        return None;
    }
    DISPLAYS.get(now).copied()
}
