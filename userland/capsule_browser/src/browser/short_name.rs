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

//! What a page reached by a short .anyone name must say. A short name is
//! weaker than the full address, since the list the Anyone DNS services sign
//! decides where it points, so the browser shows net.anon's own words next to
//! the address whenever the page's host is one. The test is the route link's
//! own, so the terminal and the browser cannot disagree on which names are
//! short.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::widgets::{Rect, Toast, Tone};

pub use nonos_route_link::SHORT_NOTICE;

/// The notice for the page at `url`, when its host is a short .anyone name.
pub fn notice_for(url: &str) -> Option<&'static str> {
    let parsed = crate::browser::url::parse(url)?;
    nonos_route_link::is_short_anyone(&parsed.host).then_some(SHORT_NOTICE)
}

/// Lines the notice may take, enough to hold it whole on the narrowest page.
pub const NOTICE_LINES: u32 = 8;

/// The notice as a `Warn` toast across the top of a page `page_w` drawing
/// pixels wide, `SPACE.sm` in from its edges, and where it is drawn.
pub fn notice_panel(text: &str, page_w: u32) -> (Toast<'_>, Rect) {
    let s = scale();
    let pad = s.px(u32::from(SPACE.sm));
    let room = page_w.saturating_sub(pad * 2);
    let logical = room.saturating_mul(4) / s.quarters().max(1);
    let toast = Toast::new(text).tone(Tone::Warn).max_width(logical).text_lines(NOTICE_LINES);
    let (w, h) = toast.size();
    (toast, Rect { x: pad, y: pad, w: w.min(room), h })
}
