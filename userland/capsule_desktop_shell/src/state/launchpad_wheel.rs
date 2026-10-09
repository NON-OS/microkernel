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

//! The wheel over the Launchpad turns its pages. The grid holds whole pages,
//! not a scrolled strip, so notches are added up and every three turn one
//! page: a mouse turns a page per three clicks, and a touchpad's two-finger
//! swipe, which posts a notch per 1/32 of the pad, turns one for about six
//! millimetres of travel rather than racing through them all. A notch away
//! from the user (positive, as every pointer driver posts it) goes back
//! toward the first page, as it scrolls a list toward its top. Turning
//! stops at the first and last page and what was added up past them is
//! dropped, so the way back starts at once.

/// Notches that turn one page.
pub const NOTCHES_PER_PAGE: i32 = 3;

/// The most notches one event is taken for, as the apps take it.
const MAX_NOTCHES: i32 = 10;

#[derive(Clone, Copy, Default)]
pub struct PageWheel {
    held: i32,
}

impl PageWheel {
    /// The page a wheel `delta_y` leaves the Launchpad on, from `page` of
    /// `pages`.
    pub fn turn(&mut self, page: usize, pages: usize, delta_y: i32) -> usize {
        let last = pages.saturating_sub(1);
        let mut page = page.min(last);
        if delta_y == 0 {
            return page;
        }
        // A change of direction starts afresh: what was added up the other
        // way would otherwise eat the first notches back.
        if self.held != 0 && (self.held > 0) != (delta_y > 0) {
            self.held = 0;
        }
        self.held += delta_y.clamp(-MAX_NOTCHES, MAX_NOTCHES);
        while self.held >= NOTCHES_PER_PAGE {
            if page == 0 {
                self.held = 0;
                break;
            }
            page -= 1;
            self.held -= NOTCHES_PER_PAGE;
        }
        while self.held <= -NOTCHES_PER_PAGE {
            if page >= last {
                self.held = 0;
                break;
            }
            page += 1;
            self.held += NOTCHES_PER_PAGE;
        }
        page
    }

    /// Forget what was added up; the Launchpad opened or closed.
    pub fn reset(&mut self) {
        self.held = 0;
    }
}
