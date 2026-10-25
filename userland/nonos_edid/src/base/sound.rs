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

//! The bar every detailed timing clears before the crate hands it out,
//! wherever it came from: a base block or CTA-861 descriptor, or a
//! DisplayID type I or VII record. A monitor is untrusted input, so a
//! timing nobody could scan out never reaches a mode list, and a driver
//! never programs one from bytes that only look like a timing.

use super::dtd::Detailed;

/// The slowest frame (or field) rate taken, in millihertz: 1 Hz. Below
/// it the totals are too large for the clock to be a picture.
pub const REFRESH_MIN_MHZ: u32 = 1_000;

/// The fastest taken: 1 kHz. Above it the totals are too small for the
/// clock. Both are this crate's bounds, not figures from a standard.
pub const REFRESH_MAX_MHZ: u32 = 1_000_000;

impl Detailed {
    /// True when the timing could be a picture: a clock, a picture,
    /// syncs that last, each front porch and sync pulse inside its
    /// blanking, and a rate between the two bounds above.
    pub fn sound(&self) -> bool {
        // No zero clock test of its own: the rate floor already asks
        // for a clock.
        let shown = self.h_active > 0 && self.v_active > 0;
        let syncs = self.h_sync > 0 && self.v_sync > 0;
        let h_fits = self.h_front.saturating_add(self.h_sync) <= self.h_blank;
        let v_fits = self.v_front.saturating_add(self.v_sync) <= self.v_blank;
        let rate = (REFRESH_MIN_MHZ..=REFRESH_MAX_MHZ).contains(&self.refresh_mhz());
        shown && syncs && h_fits && v_fits && rate
    }
}
