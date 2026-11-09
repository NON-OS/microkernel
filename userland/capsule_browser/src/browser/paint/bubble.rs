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

use alloc::string::ToString;

use nonos_app_skeleton::PaintBuffer;

use crate::browser::omnibox::geometry::bubble_at;
use crate::browser::url;

/* The address of the link under the pointer, in a tooltip at the page's
 * bottom-left corner, at most half the window wide; with no link under it,
 * a notice for the reader if one is up, which may take the window's width:
 * a page's alert is read whole where it fits. */
pub(super) fn paint(state: &crate::browser::state::State, fb: &mut PaintBuffer) {
    let (text, room) = match (state.ui.hover_href.as_deref(), state.ui.notice.as_ref()) {
        (Some(href), _) => match state.base.as_ref() {
            Some(b) => (url::join(b, href), fb.width / 2),
            None => (href.to_string(), fb.width / 2),
        },
        (None, Some((line, _))) => (line.clone(), fb.width),
        (None, None) => return,
    };
    let (tip, at) = bubble_at(&text, room, fb.width, fb.height);
    tip.draw(fb, at);
}
