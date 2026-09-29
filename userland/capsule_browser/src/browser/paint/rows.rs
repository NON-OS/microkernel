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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::{Content, Fragment};
use crate::browser::layout::hit_screen::frag_screen_y;
use crate::browser::omnibox::geometry::CONTENT_TOP;
use crate::browser::omnibox::{band_safe, extend_rows};
use crate::browser::state::State;

/* Repaint viewport rows [y0, y1) as a full paint does: the band grows over
 * boxes crossing its edges, then the page painter draws into it with the
 * scroll moved by the band's top. False without a box page, or when a fixed
 * or sticky box would land elsewhere than a full paint puts it. */
pub(super) fn paint_rows(state: &mut State, fb: &mut PaintBuffer, y0: u32, y1: u32) -> bool {
    let Some(doc) = state.box_doc.as_ref() else {
        return false;
    };
    if !band_safe(doc.frags.iter().map(|f| (f.fixed, f.sticky.is_some()))) {
        return false;
    }
    let view_h = fb.height.saturating_sub(CONTENT_TOP) as i32;
    let scroll = state.scroll as i32;
    let spans: Vec<(i32, i32)> = doc.frags.iter().filter_map(|f| reach(f, scroll)).collect();
    let (a, b) = extend_rows(&spans, y0 as i32, y1 as i32, view_h);
    if b <= a {
        return true;
    }
    let mut sub = fb.sub(0, a as u32, fb.width, CONTENT_TOP + (b - a) as u32);
    state.scroll = state.scroll.saturating_add(a as u32);
    if let Some(doc) = state.box_doc.as_ref() {
        super::box_page::paint(state, doc, &mut sub);
    }
    state.scroll = state.scroll.saturating_sub(a as u32);
    true
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
