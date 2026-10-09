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

//! Choosing text: a press starts, a drag extends, a double click takes a
//! word and a triple click a line, and Alt makes it a block.

use crate::term::select::Selection;
use crate::term::terminal::Terminal;
use nonos_app_skeleton::{InputEvent, MOD_ALT};
use nonos_vt::Pos;

/// Presses closer together than this count as a double or triple click.
const MULTI_CLICK_NS: u64 = 400_000_000;

impl Terminal {
    pub(super) fn press(&mut self, event: &InputEvent, pos: Pos) {
        let p = &mut self.ptr;
        let again =
            event.timestamp_ns.saturating_sub(p.last_ns) < MULTI_CLICK_NS && p.last_pos == pos;
        p.clicks = if again { p.clicks % 3 + 1 } else { 1 };
        (p.last_ns, p.last_pos, p.start) = (event.timestamp_ns, pos, pos);
        p.block = event.flags & MOD_ALT != 0;
        p.selecting = true;
        let clicks = p.clicks;
        let state = self.cur();
        let vt = &state.scrollback.vt;
        let span = match clicks {
            2 => Some(vt.word_at(pos)),
            3 => Some(vt.line_bounds(pos)),
            _ => None,
        };
        state.sel = span.map(|(a, b)| Selection { anchor: a, head: b, block: false });
    }

    pub(super) fn drag(&mut self, pos: Pos) {
        let (start, block, clicks) = (self.ptr.start, self.ptr.block, self.ptr.clicks);
        let state = self.cur();
        if pos == start && state.sel.is_none() {
            return;
        }
        let anchor = state.sel.map(|x| x.anchor).filter(|_| clicks > 1).unwrap_or(start);
        state.sel = Some(Selection { anchor, head: pos, block });
    }
}
