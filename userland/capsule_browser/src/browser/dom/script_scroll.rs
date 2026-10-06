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

//! A page's own scrolling: `window.scrollTo`, `scrollBy` and `scroll`, and
//! an element's `scrollIntoView`.
//!
//! These were not defined, so the call threw and took the rest of the
//! script with it: a "back to top" button, a form that brings its first
//! error into view, a page that opens at a saved place. The script moves
//! `scroll_y` here, where its next read of `scrollY` sees it at once; the
//! browser moves the window to match when the script returns and fires
//! `scroll`, as for the reader's own scroll (event::scroll_by).

use super::tree::Dom;

/// Where `scrollIntoView` puts the element, its `block` option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    /// The element's top at the window's top: `true`, no argument, or
    /// `block: 'start'`.
    Start,
    Center,
    /// The element's bottom at the window's bottom: `false` or `'end'`.
    End,
    /// Moved as little as brings it in, and not at all when it is in.
    Nearest,
}

impl Block {
    /// The code the bindings pass: 1 center, 2 end, 3 nearest, else start.
    pub fn from_code(code: i32) -> Block {
        match code {
            1 => Block::Center,
            2 => Block::End,
            3 => Block::Nearest,
            _ => Block::Start,
        }
    }
}

impl Dom {
    /// Scroll the page to `y`, within what it can scroll: from the top to
    /// where its bottom meets the window's. Answers where it is now.
    ///
    /// Before the page was first laid out there is no height to hold it
    /// to, so the wish is kept as asked and the browser holds it to the
    /// layout when it follows (event::scroll_by): a page that puts the
    /// reader back where they were as it loads is not sent to the top.
    pub fn script_scroll(&mut self, y: i64) -> u32 {
        let max = match self.rects.is_empty() {
            true => u32::MAX as i64,
            false => self.content_h.saturating_sub(self.viewport.1) as i64,
        };
        self.scroll_y = y.clamp(0, max) as u32;
        self.scroll_y
    }

    /// The offset that brings `node` into view as `block` asks, or None
    /// for a node that was not laid out (one not displayed), which a
    /// browser does not scroll to.
    pub fn into_view_y(&self, node: usize, block: Block) -> Option<i64> {
        let r = *self.rects.get(node)?;
        if r == [0; 4] {
            return None;
        }
        let (top, h) = (r[1] as i64, r[3] as i64);
        let (cur, view) = (self.scroll_y as i64, self.viewport.1 as i64);
        let end = top + h - view;
        Some(match block {
            Block::Start => top,
            Block::End => end,
            Block::Center => top + h / 2 - view / 2,
            Block::Nearest if top >= cur && top + h <= cur + view => cur,
            Block::Nearest if top < cur || h > view => top,
            Block::Nearest => end,
        })
    }
}
