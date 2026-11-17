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

use crate::browser::layout::boxmodel::{BoxDocument, Content, Fragment};

use super::device_parts::{layer, object_fit, shadow};
use super::device_units::to_device;

/* The page is laid out in CSS px; the window draws in device px, `q`
 * quarters of a device px to the CSS px (4 is 1x). The laid-out document
 * is carried into device px once, here, so paint, hit testing and the
 * window's scroll all work in the pixels they draw. Box edges round on
 * their own, so boxes that touch in CSS px still touch on the device. */

/// Carry `doc`, laid out in CSS px, into device px at `q` quarters.
pub fn doc_to_device(doc: &mut BoxDocument, q: u32) {
    if q == 4 {
        return;
    }
    doc.frags.iter_mut().for_each(|f| frag(f, q));
    doc.content_h = to_device(doc.content_h.min(i32::MAX as u32) as i32, q).max(0) as u32;
    if let Some(c) = doc.canvas_bg_image.as_mut() {
        c.layer = layer(c.layer, q);
    }
}

fn frag(f: &mut Fragment, q: u32) {
    let d = |v: i32| to_device(v, q);
    let du = |v: u16| d(v as i32).clamp(0, u16::MAX as i32) as u16;
    let (x1, y1) = (d(f.x.saturating_add(f.w)), d(f.y.saturating_add(f.h)));
    (f.x, f.y) = (d(f.x), d(f.y));
    (f.w, f.h) = (x1.saturating_sub(f.x), y1.saturating_sub(f.y));
    f.border = f.border.map(|b| d(b.min(i32::MAX as u32) as i32).max(0) as u32);
    f.clip = f.clip.map(|c| c.map(d));
    (f.clip_r, f.radius) = (f.clip_r.map(du), f.radius.map(du));
    f.sticky = f.sticky.map(|(a, t)| (d(a), d(t)));
    f.shadow = f.shadow.map(|s| shadow(s, q));
    f.bg_layer = layer(f.bg_layer, q);
    match &mut f.content {
        Content::Text { px, spacing, .. } => {
            *px = *px * q as f32 / 4.0;
            *spacing = *spacing * q as f32 / 4.0;
        }
        Content::Image { fit, .. } => *fit = object_fit(*fit, q),
        Content::None => {}
    }
}
