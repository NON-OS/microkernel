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
use super::tuning::{NATURAL_SCROLL, SCROLL_JUMP_DIV, SCROLL_NOTCHES_PER_PAD};
use super::types::TouchGesture;

impl TouchGesture {
    /// One report of a two-finger touch. The scroll follows one finger: the
    /// first one seen, by its contact identifier when the pad gives one, so
    /// the reports of the other finger in hybrid mode neither move it nor
    /// re-anchor it. Whole notches are posted and the rest is kept for the
    /// next report.
    pub(super) fn scroll(&mut self, id: Option<u32>, tip: bool, y: u32, y_max: u32) -> Option<i32> {
        if !self.scrolling {
            if tip {
                self.scrolling = true;
                self.scroll_id = id;
                self.scroll_y = y;
            }
            return None;
        }
        let mine = match (self.scroll_id, id) {
            (Some(a), Some(b)) => a == b,
            _ => true,
        };
        if !mine || !tip {
            return None;
        }
        let dy = y as i32 - self.scroll_y as i32;
        if dy.unsigned_abs() > y_max / SCROLL_JUMP_DIV {
            self.scroll_y = y;
            return None;
        }
        let step = (y_max / SCROLL_NOTCHES_PER_PAD).max(1) as i32;
        let notches = dy / step;
        if notches == 0 {
            return None;
        }
        self.scroll_y = (self.scroll_y as i32 + notches * step) as u32;
        // Fingers moving down the pad (y growing) scroll the view down,
        // which is a negative wheel, unless natural scrolling is on.
        Some(if NATURAL_SCROLL { notches } else { -notches })
    }
}
