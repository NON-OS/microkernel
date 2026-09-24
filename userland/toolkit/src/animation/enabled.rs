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

//! Whether motion is drawn, or skipped straight to the end.

use core::sync::atomic::{AtomicBool, Ordering};

/*
 * Turning animation off does not cancel what the animation was for. A fade that
 * does not run still has to leave the thing faded, and a slide that does not run
 * still has to leave the panel where it was sliding to: the alternative is a
 * setting that breaks every transition instead of shortening it.
 *
 * So this is read in `Animation::start`, which is the one door every animation in
 * the toolkit goes through, and a disabled animation arrives at its destination
 * immediately rather than never leaving.
 *
 * On by default. An application that never hears from the settings store behaves
 * the way the desktop always has.
 */
static ENABLED: AtomicBool = AtomicBool::new(true);

/// Whether animations should play.
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Turn motion on or off for every animation this process starts from now on.
///
pub fn set_enabled(value: bool) {
    ENABLED.store(value, Ordering::Relaxed);
}
