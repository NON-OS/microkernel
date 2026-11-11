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

use nonos_app_skeleton::measure_ttf;

use super::State;
use crate::browser::omnibox::geometry::{pill_text, text_px};
use crate::browser::omnibox::text_offset;

impl State {
    /* Scroll the address text so the caret stays inside the pill. */
    pub fn fit_text(&mut self) {
        let ed = &self.ui.omnibox;
        let caret_px = measure_ttf(&ed.text[..ed.caret], text_px());
        let full = measure_ttf(&ed.text, text_px());
        let w = pill_text(self.layout.toolbar, self.ui.truncated).w as i32;
        self.ui.text_off = text_offset(caret_px, full, w, self.ui.text_off);
    }

    /* How far the start page's search pill, `w` pixels of text wide,
     * scrolls the address text to keep the caret in view. */
    pub fn start_text_off(&self, w: i32) -> i32 {
        let ed = &self.ui.omnibox;
        let caret_px = measure_ttf(&ed.text[..ed.caret], text_px());
        text_offset(caret_px, measure_ttf(&ed.text, text_px()), w, 0)
    }
}
