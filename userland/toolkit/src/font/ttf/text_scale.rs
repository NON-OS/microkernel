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

//! How large the chosen text size makes every string the desktop draws.

use core::sync::atomic::{AtomicU32, Ordering};

/// The scale as a fraction of this, so the live value is an integer and needs no
/// float atomic. 1024 is unchanged.
pub const ONE: u32 = 1024;

/*
 * One factor per label in `FONT_SIZE_LABELS`: Tiny, Small, Normal, Large, Huge.
 *
 * Anchored at index 1, not index 2. The policy store's default is 1, so that is
 * what every window on this desktop is already drawn at, and a table anchored at
 * "Normal" would change the size of every label on every screen the first time
 * this setting was wired up. The names are the store's; the appearance people
 * already have is the baseline.
 *
 * Steps of roughly an eighth. Smaller than that is not visible enough to be
 * worth a setting, and larger starts pushing text out of rows whose height is a
 * fixed number of pixels.
 */
const STEPS: [u32; 5] = [922, ONE, 1152, 1280, 1434];

static SCALE: AtomicU32 = AtomicU32::new(ONE);

/// Apply the stored text size. A value the table does not have leaves the scale
/// alone rather than resetting it: a store that answered with something this
pub fn set_steps(steps: u8) {
    if let Some(scale) = STEPS.get(steps as usize) {
        SCALE.store(*scale, Ordering::Relaxed);
    }
}

/// The factor for a stored value, for callers that need it without setting it.
pub fn scale_of(steps: u8) -> u32 {
    match STEPS.get(steps as usize) {
        Some(scale) => *scale,
        None => ONE,
    }
}

/// The live factor.
pub fn scale() -> u32 {
    SCALE.load(Ordering::Relaxed)
}

/// `px` at the chosen text size.
pub fn apply(px: f32) -> f32 {
    px * (scale() as f32) / (ONE as f32)
}
