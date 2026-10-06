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

use alloc::vec::Vec;

use crate::browser::layout::boxmodel::{Content, Fragment};
use crate::browser::layout::hit_screen::frag_screen_y;
use crate::browser::omnibox::{band_safe, extend_rows};
use crate::browser::state::State;

/// The viewport rows [a, b) a repaint of rows [y0, y1) draws: the band
/// grown over boxes crossing its edges. None without a box page, or when a
/// fixed or sticky box would land elsewhere than a full paint puts it, and
/// the page is then painted whole.
pub(crate) fn band_rows(state: &State, y0: i32, y1: i32, view_h: i32) -> Option<(i32, i32)> {
    let doc = state.box_doc.as_ref()?;
    if !band_safe(doc.frags.iter().map(|f| (f.fixed, f.sticky.is_some()))) {
        return None;
    }
    let scroll = state.scroll as i32;
    let spans: Vec<(i32, i32)> = doc.frags.iter().filter_map(|f| reach(f, scroll)).collect();
    Some(extend_rows(&spans, y0, y1, view_h))
}

/* The rows a fragment draws that a band edge must not cut: text and images
 * draw whole or not at all, and no shadow layer is clipped to the page area.
 * Plain fills clip to any band, so a tall wrapper does not grow the band. */
fn reach(f: &Fragment, scroll: i32) -> Option<(i32, i32)> {
    let top = frag_screen_y(f.y, f.fixed, f.sticky, scroll);
    let whole = !matches!(f.content, Content::None) || f.bg_image.is_some();
    let (mut up, mut down) = (0, 0);
    if let Some(s) = f.shadow.as_ref() {
        for l in &s.layers[..s.n as usize] {
            let blur = (l.blur as i32).clamp(0, 40) + (l.spread as i32).max(0);
            up = up.max(blur - l.dy as i32);
            down = down.max(blur + l.dy as i32);
        }
    } else if !whole {
        return None;
    }
    if let Content::Text { px, .. } = &f.content {
        let pad = *px as i32 / 2 + 2;
        (up, down) = (up.max(pad), down.max(pad));
    }
    Some((top.saturating_sub(up), top.saturating_add(f.h).saturating_add(down)))
}
