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

use alloc::collections::BTreeSet;
use alloc::string::String;

use super::revival::MAX_THRASH;
use super::store_entry::Status;
use crate::browser::layout::boxmodel::Content;
use crate::browser::state::State;

/// Queue again the evicted images of every box on or near the screen (one
/// viewport above and below), as a scroll or a new layout brings them into
/// view. Images far off screen stay evicted, so the budget is not churned.
/// Sources resolve through the store's join memo, not a URL join each.
pub fn requeue_visible(state: &mut State) {
    let Some(base) = state.base.clone() else { return };
    let (top, vh) = (state.scroll as i32, state.viewport_h.max(1) as i32);
    let near = |y: i32, h: i32| y.saturating_add(h) > top - vh && y < top.saturating_add(2 * vh);
    let mut shown: BTreeSet<String> = BTreeSet::new();
    for f in state.box_doc.iter().flat_map(|d| d.frags.iter()) {
        if !(f.fixed || near(f.y, f.h)) {
            continue;
        }
        let src = match &f.content {
            Content::Image { src, .. } => Some(src.as_str()),
            _ => f.bg_image.as_deref(),
        };
        if let Some(s) = src {
            shown.insert(state.images.with_abs(&base, s, |abs: &str| String::from(abs)));
        }
    }
    for (url, e) in state.images.entries.iter_mut() {
        e.near = shown.contains(url);
        let r = &mut e.revival;
        if !e.near || !matches!(e.status, Status::Evicted) {
            continue;
        }
        if r.thrash {
            if r.refetched >= MAX_THRASH {
                continue;
            }
            r.refetched += 1;
        }
        r.thrash = false;
        e.status = Status::Pending;
        if !state.image_queue.contains(url) {
            state.image_queue.push(url.clone());
        }
    }
}
