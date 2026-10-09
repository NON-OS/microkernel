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

use alloc::string::String;

use crate::browser::omnibox::Change;

use super::State;

/* How long a line told to the reader stays up. */
const TOLD_MS: i64 = 6_000;

impl State {
    /* Tell the reader `line` whether or not a page is on screen. The status
     * line is drawn only while there is no page, so a line put only there
     * was lost whenever one was; the notice bubble at the page's foot shows
     * it too, until it is taken down (event::script_stop). */
    pub fn tell(&mut self, line: String) {
        self.status = line.clone();
        self.ui.notice = Some((line, nonos_libc::mk_uptime_ms() + TOLD_MS));
        self.mark(Change::Bubble);
    }
}
