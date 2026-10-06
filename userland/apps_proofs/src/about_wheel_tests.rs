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

//! About's screens scroll on the wheel: three arrow steps a notch, toward
//! the top on a notch away, and never past the top or the last content.

use crate::about_scroll::scroll_wheel::{wheel_to, WHEEL_STEP};
use crate::about_scroll::ui::metrics::SCROLL_STEP;

#[test]
fn a_notch_is_three_arrow_steps() {
    assert_eq!(WHEEL_STEP, 3 * SCROLL_STEP);
    assert_eq!(wheel_to(0, 1000, -1), WHEEL_STEP, "a notch toward the user reads on");
    assert_eq!(wheel_to(500, 1000, 1), 500 - WHEEL_STEP, "a notch away goes back up");
}

#[test]
fn the_screen_stops_at_its_top_and_at_its_last_content() {
    assert_eq!(wheel_to(10, 1000, 1), 0);
    assert_eq!(wheel_to(990, 1000, -1), 1000);
    assert_eq!(wheel_to(1000, 1000, -3), 1000);
    assert_eq!(wheel_to(0, 0, -1), 0, "a screen that fits its pane does not move");
}
