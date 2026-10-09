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

//! Where each line of the page sits, and how far the wheel scrolls it. The
//! painter and the scroll limit both walk `place`, so the page stops with its
//! last line on screen, not short of it and not past it.

use nonos_app_skeleton::scroll::{wheel_px, WHEEL_LINES};

use super::layout::{gap, line_height, Line, Style};

/// The margin above the first line, and below the last when scrolled to it.
pub const TOP: i32 = 20;

/// Calls `f` with each line's top, from the top of the unscrolled page, and
/// returns where the last line ends.
pub fn place(lines: &[Line], mut f: impl FnMut(i32, &Line)) -> i32 {
    let mut y = TOP;
    let mut started = false;
    for line in lines {
        if line.lead && started {
            y += gap(line.style);
        }
        f(y, line);
        y += line_height(line.style);
        started = true;
    }
    y
}

/// How far a page `view_h` tall scrolls: until its last line sits the top
/// margin above the bottom edge. A page that fits does not scroll.
pub fn max_scroll(lines: &[Line], view_h: u32) -> u32 {
    let end = place(lines, |_, _| {}) + TOP;
    end.saturating_sub(view_h as i32).max(0) as u32
}

/// Pixels a notch moves the page: three lines of body text.
pub fn wheel_step() -> u32 {
    line_height(Style::Body) as u32 * WHEEL_LINES as u32
}

/// The scroll offset after a wheel `delta_y`; a notch away from the user goes
/// up.
pub fn wheel(scroll: u32, lines: &[Line], view_h: u32, delta_y: i32) -> u32 {
    wheel_px(scroll, delta_y, wheel_step(), max_scroll(lines, view_h))
}
