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

use super::decode::{decode_to, natural};
use super::plan::{target, MAX_RASTER_PX};
use super::plan_vector::{box_target, vector_target};
use super::store::Store;
use super::svg::{is_svg, MAX_SIDE};

/// Decode a fetched image body and record the outcome against `url`. The
/// raster is kept at the size of the largest box it is drawn into (noted
/// at enqueue), not its natural size, and the budget is made (evicting the
/// least recently painted rasters) before anything is allocated; a raster
/// the budget still cannot hold is decoded smaller.
pub fn ingest(store: &mut Store, url: &str, body: &[u8]) {
    let Some(nat) = natural(body) else {
        store.set_failed(url);
        return;
    };
    let hint = store.hint(url);
    let want = if is_svg(body) && store.only_in_img_box(url) {
        box_target(nat, hint, MAX_SIDE)
    } else if is_svg(body) {
        vector_target(nat, hint, MAX_SIDE)
    } else {
        target(nat, hint, MAX_RASTER_PX)
    };
    let need = want.0 as usize * want.1 as usize * 4;
    let free = store.room(url, need) / 4;
    let size = if free >= need / 4 { want } else { target(nat, want, free.max(1) as u64) };
    match decode_to(body, size) {
        Ok(d) => {
            store.set_natural(url, nat);
            store.set_ready(url, d);
        }
        Err(_) => store.set_failed(url),
    }
}

/// Record the box a source will be drawn into before ingest, so it decodes
/// at its display size. The device path does this through the fetch queue;
/// the host render harness has no queue and calls this directly.
#[cfg(feature = "harness")]
pub fn note_size(store: &mut Store, url: &str, w: u32, h: u32) {
    store.mark_pending(url);
    store.note_hint(url, w, h);
}

/// note_size for a source drawn only as the content of an <img> box, as the
/// device queue records it: vector art then rasterizes at the box's shape.
#[cfg(feature = "harness")]
pub fn note_img_size(store: &mut Store, url: &str, w: u32, h: u32) {
    note_size(store, url, w, h);
    store.note_drawn_as(url, super::store_hint::IMG_BOX);
}
