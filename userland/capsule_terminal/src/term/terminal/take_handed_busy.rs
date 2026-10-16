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

//! Saying a handed command found no tab free to take it.

use crate::term::terminal::types::Terminal;
use crate::term::util::copy_into;

const BUSY: &[u8] = b": every tab is busy, close one and click the tile again";

impl Terminal {
    pub(super) fn say_busy(&mut self, line: &[u8]) {
        let mut msg = [0u8; 128];
        let head = line.len().min(msg.len() - BUSY.len());
        let mut n = copy_into(&mut msg, &line[..head]);
        n += copy_into(&mut msg[n..], BUSY);
        let state = self.cur();
        state.scrollback.push_line(&msg[..n]);
        state.scrollback.jump_bottom();
    }
}
