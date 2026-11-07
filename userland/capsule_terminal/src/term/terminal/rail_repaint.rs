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

//! Whether a rail sample asks for the window to be drawn again. The rail
//! samples once a second whether it is on screen or not, so its chart has a
//! history when it is opened, and that second is also when the banner a new
//! tab shows draws its uptime again. Once the banner is gone and the rail is
//! hidden, a sample changes no pixel, and drawing the window for it made the
//! compositor take the whole window again every second. While the banner is
//! all a sample redraws, only its uptime row has changed, and that row alone
//! is presented (damage.rs).

use crate::term::prefs::RAIL_VISIBLE;

/// A sample redraws the window while the rail or the banner is on screen.
pub fn sample_repaints(sampled: bool, rails: u8, banner: bool) -> bool {
    sampled && (rails & RAIL_VISIBLE != 0 || banner)
}

/// A sample whose only change on screen is the banner's uptime row: the
/// banner is shown and the rail, which would change too, is hidden.
pub fn uptime_row_only(sampled: bool, rails: u8, banner: bool) -> bool {
    sampled && banner && rails & RAIL_VISIBLE == 0
}
