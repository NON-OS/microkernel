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

use super::css_cut::cut;

/* The most stylesheet text one page keeps, all fetched sheets together. */
pub const MAX_PAGE_CSS: usize = 8 * 1024 * 1024;

/* One stylesheet fetch has finished: its text (None when it failed or was
empty) joins the page's CSS, and the answer is whether to lay the page
out now. That is only once no sheet is left to fetch: the pump takes a
sheet off `queue` when it starts it and imports join `queue` before
this runs, so the last sheet to finish, fetched or failed, relayouts
the page exactly once and the render-blocking hold lasts until every
sheet has been tried. */
pub fn sheet_done<T>(page_css: &mut String, queue: &[T], text: Option<&str>) -> bool {
    if let Some(text) = text {
        fold(page_css, text);
    }
    queue.is_empty()
}

/* Appends a sheet after a newline. One that would take the page past
`MAX_PAGE_CSS` is cut after its last whole top-level rule that fits,
so a large framework sheet keeps its first rules instead of vanishing;
returns how many of its bytes were kept. */
pub fn fold(page_css: &mut String, sheet: &str) -> usize {
    let room = MAX_PAGE_CSS.saturating_sub(page_css.len() + 1);
    let keep = if sheet.len() <= room { sheet.len() } else { cut(sheet, room) };
    if keep > 0 {
        page_css.push('\n');
        page_css.push_str(&sheet[..keep]);
    }
    keep
}
