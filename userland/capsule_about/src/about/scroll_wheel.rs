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

//! Where the wheel takes the screen on show. A notch is three arrow steps,
//! as a notch is three lines in the text views, and the screen stops at its
//! top and at the last of its content.

use nonos_app_skeleton::scroll::{wheel_px, WHEEL_LINES};

use super::ui::metrics::SCROLL_STEP;

/// Pixels one notch moves the screen.
pub const WHEEL_STEP: u32 = SCROLL_STEP * WHEEL_LINES as u32;

/// The scroll offset after a wheel `delta_y`, for a screen that scrolls as
/// far as `max`.
pub fn wheel_to(scroll: u32, max: u32, delta_y: i32) -> u32 {
    wheel_px(scroll, delta_y, WHEEL_STEP, max)
}
