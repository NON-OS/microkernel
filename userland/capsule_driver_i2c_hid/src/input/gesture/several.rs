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
use super::types::{TouchActions, TouchGesture};
use crate::hid::TouchSample;

impl TouchGesture {
    /// Two fingers or more scroll and never move the cursor, and after them
    /// nothing moves it until every finger has lifted. True when the frame
    /// was theirs.
    pub(super) fn several(
        &mut self,
        s: &TouchSample,
        y: u32,
        y_max: u32,
        act: &mut TouchActions,
    ) -> bool {
        // A precision touchpad in hybrid mode sends one finger per report and
        // the frame's contact count only in the first; the others say zero.
        // A report that says zero with its finger down belongs to the frame
        // the last count described.
        if s.contacts > 0 {
            self.frame_contacts = s.contacts;
        } else if !s.tip {
            self.frame_contacts = 0;
        }
        let contacts = if s.contacts == 0 && s.tip { self.frame_contacts } else { s.contacts };

        if contacts >= 2 {
            self.multi_touch = true;
            if let Some(wheel) = self.scroll(s.contact_id, s.tip, y, y_max) {
                act.wheel = wheel;
            }
            return true;
        }
        self.scrolling = false;
        self.scroll_id = None;

        // PTP hybrid reporting alternates which contact a report carries, and
        // only the first report of a frame set carries the true contact count.
        // While several fingers were down, the single-contact frames in
        // between alternate finger positions and would teleport the cursor;
        // hold all movement until every finger has lifted.
        if self.multi_touch {
            if !s.tip {
                self.multi_touch = false;
            }
            return true;
        }
        false
    }
}
