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

use crate::browser::css::Size;

use super::geom::replaced_clamp::clamp_replaced;
use super::image_box::image_box;
use super::tree::BoxNode;

/// The content size of an image box at most `avail` px wide, in a block of
/// definite height `cb_h` if any. A side comes from CSS, else from the
/// width/height attribute; a missing side follows the aspect ratio (the
/// natural one, else the attributes' w/h, else aspect-ratio), and with no
/// side given at all the natural size stands. A width that CSS did not set
/// stays within max-width and `avail`, the height following it by the
/// ratio, and min/max sizes then bound it (geom::replaced_clamp). With
/// neither a size nor a ratio known, the default image box.
pub(super) fn replaced_size(n: &BoxNode, avail: i32, cb_h: Option<i32>) -> (i32, i32) {
    let s = &n.style;
    let [aw, ah] = n.aux.attr.map(|a| a.map(|v| v as i32));
    let nat = n.aux.natural.filter(|&(w, h)| w > 0 && h > 0);
    let attr_ratio = aw.zip(ah).filter(|&(w, h)| w > 0 && h > 0).map(|(w, h)| w as f32 / h as f32);
    let ratio = nat.map(|(w, h)| w as f32 / h as f32).or(attr_ratio).or(s.aspect);
    let css_w = s.width.resolve(avail.max(0));
    let css_h = s.height.definite_px().or_else(|| cb_h.and_then(|b| s.height.resolve(b)));
    /* The height attribute is only a hint while no ratio can give the height. */
    let w = css_w.or(aw);
    let h = css_h.or(if ratio.is_some() { None } else { ah });
    let by = |v: i32, k: f32| (v as f32 * k + 0.5) as i32;
    let w_set = w.is_some();
    let (w, derived_h, h) = match (w, h, ratio, nat) {
        (Some(w), Some(h), _, _) => (w, false, h),
        (Some(w), None, Some(r), _) => (w, true, by(w, 1.0 / r)),
        (None, Some(h), Some(r), _) => (by(h, r), false, h),
        (None, None, Some(_), Some((nw, nh))) => (nw as i32, true, nh as i32),
        (w, h, r, _) => {
            let (fw, fh) = image_box(s, avail);
            let w = w.unwrap_or(fw);
            (w, h.is_none(), h.or(r.map(|r| by(w, 1.0 / r))).unwrap_or(fh))
        }
    };
    let set = (w_set, !derived_h);
    let (w, h) = clamp_replaced(s, (w, h), set, ratio, (avail, cb_h));
    (w.max(0), h.max(0))
}

/// Whether this image's box depends on a natural size not yet known, so its
/// arrival should lay the page out again.
pub(super) fn awaits_natural(n: &BoxNode) -> bool {
    let sized = n.style.width != Size::Auto && n.style.height != Size::Auto;
    n.aux.natural.is_none() && !sized && !n.aux.attr.iter().all(Option::is_some)
}
