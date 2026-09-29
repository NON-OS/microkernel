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
use alloc::vec::Vec;

use crate::browser::layout::boxmodel::Content;
use crate::browser::state::State;
use crate::browser::url;

use super::store_hint::{BG_BOX, IMG_BOX};

/* Collect every image source in the current page, resolve it against the
 * page base, and queue the ones not already known for fetching. */
pub fn enqueue_from_doc(state: &mut State) {
    let Some(base) = state.base.clone() else { return };
    let mut fresh: Vec<String> = Vec::new();
    let mut hints: Vec<(String, u32, u32, u8)> = Vec::new();
    if let Some(doc) = state.box_doc.as_ref() {
        for f in &doc.frags {
            let (w, h) = (f.w.max(0) as u32, f.h.max(0) as u32);
            if let Content::Image { src, .. } = &f.content {
                queue(&base, src, &state.images, &mut fresh);
                /* A box laid out before the natural size was known has a
                 * stand-in size; decoding to it would blur the image once
                 * the real box arrives, so it notes no hint. */
                let abs = url::join(&base, src);
                if !doc.awaits(&abs, Some(&base)) {
                    hints.push((abs, w, h, IMG_BOX));
                }
            }
            if let Some(src) = f.bg_image.as_deref() {
                if !src.starts_with("linear-gradient(") && !src.starts_with("radial-gradient(") {
                    queue(&base, src, &state.images, &mut fresh);
                    hints.push((url::join(&base, src), w, h, BG_BOX));
                }
            }
        }
    }
    for u in fresh {
        state.images.mark_pending(&u);
        state.image_queue.push(u);
    }
    /* Record the displayed size of every referencing box so a vector source
     * rasterizes at the size it is drawn instead of upscaling a tiny natural. */
    for (u, w, h, how) in hints {
        state.images.note_drawn_as(&u, how);
        state.images.note_hint(&u, w, h);
    }
    super::revive::requeue_visible(state);
}

/* Resolve one source against the base and add it if not already known. */
fn queue(base: &url::Url, src: &str, images: &super::Store, fresh: &mut Vec<String>) {
    let abs = url::join(base, src);
    if !images.contains(&abs) && !fresh.contains(&abs) {
        fresh.push(abs);
    }
}
