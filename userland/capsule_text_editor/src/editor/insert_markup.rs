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

//! Insert > Link and Insert > Image. The editor holds text, so both insert
//! Markdown: `[text](address)` and `![alt](path)`. A selection becomes the
//! text and the caret lands between the parentheses, where the address goes;
//! with nothing selected the caret lands between the brackets instead.

use alloc::vec::Vec;

use super::state::State;

impl State {
    /// Replace the selection (or insert at the caret) with a Markdown link, or
    /// an image when `image`, as one undo step. False if it would not fit.
    pub fn insert_markup(&mut self, image: bool) -> bool {
        let (start, end) = self.sel_range().unwrap_or_else(|| {
            let at = self.caret.min(self.len);
            (at, at)
        });
        let text = &self.buf[start..end];
        let open = if image { &b"!["[..] } else { &b"["[..] };
        let mut ins = Vec::with_capacity(open.len() + text.len() + 3);
        ins.extend_from_slice(open);
        ins.extend_from_slice(text);
        ins.extend_from_slice(b"]()");
        let caret = if text.is_empty() { start + open.len() } else { start + ins.len() - 1 };
        if !self.apply_edit(start, end - start, &ins) {
            return false;
        }
        self.sel_anchor = None;
        self.caret = caret;
        true
    }
}
