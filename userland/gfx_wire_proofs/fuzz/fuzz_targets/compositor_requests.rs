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

#![no_main]

use compositor_proofs::requests::area::{damage_commit, Area};
use compositor_proofs::requests::content::{layer_content, layer_content_parts, CONTENT_OPAQUE};
use compositor_proofs::requests::parts::Parts;
use compositor_proofs::requests::place::scene_submit;
use compositor_proofs::requests::{capture, place_keyed, pointer, present, remove, toggles, vcp};
use libfuzzer_sys::fuzz_target;

const CONTENT_LEN: usize = 64;

fn word(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i * 4], b[i * 4 + 1], b[i * 4 + 2], b[i * 4 + 3]])
}

/// OP_LAYER_CONTENT: taken only at its own length, with a layer named, the
/// layer never its own content, no flag but opaque and the pad clear; what
/// is taken is the body's words as they were.
fn content(body: &[u8]) {
    let Some(c) = layer_content(body) else {
        if body.len() == CONTENT_LEN {
            let w = |i| word(body, i);
            let (on, handle) =
                (u64::from(w(0)) | u64::from(w(1)) << 32, u64::from(w(2)) | u64::from(w(3)) << 32);
            assert!(on == 0 || on == handle || w(6) & !CONTENT_OPAQUE != 0 || w(15) != 0);
        }
        return;
    };
    assert_eq!(body.len(), CONTENT_LEN);
    assert!(c.on != 0 && c.on != c.handle);
    let w = |i| word(body, i);
    assert_eq!((c.on as u32, (c.on >> 32) as u32), (w(0), w(1)));
    assert_eq!((c.handle as u32, (c.handle >> 32) as u32), (w(2), w(3)));
    assert_eq!((c.x, c.y), (w(4), w(5)));
    assert_eq!(c.opaque, w(6) == CONTENT_OPAQUE);
    assert_eq!(c.area, Area { x: w(7), y: w(8), width: w(9), height: w(10) });
    assert_eq!(c.view, Area { x: w(11), y: w(12), width: w(13), height: w(14) });
    assert_eq!(w(15), 0);
}

/// Every box a parts block names lies wholly inside the request's own box,
/// and there are one to four of them.
fn within(p: &Parts, bound: Area) {
    let n = p.iter().count();
    assert!((1..=4).contains(&n));
    for a in p.iter() {
        assert!(a.width != 0 && a.height != 0);
        assert!(a.x >= bound.x && a.y >= bound.y);
        assert!(u64::from(a.x) + u64::from(a.width) <= u64::from(bound.x) + u64::from(bound.width));
        assert!(
            u64::from(a.y) + u64::from(a.height) <= u64::from(bound.y) + u64::from(bound.height)
        );
    }
}

/// A damaged box clipped to a canvas lies on it, and is cut, never moved.
fn clipped(a: Area, w: u32, h: u32) {
    if let Some(c) = a.clip(w, h) {
        assert_eq!((c.x, c.y), (a.x, a.y));
        assert!(c.width != 0 && c.height != 0);
        assert!(u64::from(c.x) + u64::from(c.width) <= u64::from(w));
        assert!(u64::from(c.y) + u64::from(c.height) <= u64::from(h));
        assert!(c.width <= a.width && c.height <= a.height);
    } else {
        assert!(a.width == 0 || a.height == 0 || a.x >= w || a.y >= h);
    }
}

fuzz_target!(|data: &[u8]| {
    let (canvas, body) = data.split_at(data.len().min(4));
    let w = u32::from(u16::from_le_bytes([
        *canvas.first().unwrap_or(&0),
        *canvas.get(1).unwrap_or(&0),
    ]));
    let h =
        u32::from(u16::from_le_bytes([*canvas.get(2).unwrap_or(&0), *canvas.get(3).unwrap_or(&0)]));
    content(body);
    let mut padded = body.to_vec();
    padded.resize(CONTENT_LEN, 0);
    content(&padded);
    if let Some(a) = damage_commit(body) {
        clipped(a, w, h);
    }
    if let Some(c) = layer_content(&padded) {
        clipped(c.area, w, h);
    }
    if let Some(p) = scene_submit(body) {
        if let Some(l) = p.layer(1, w, h) {
            assert!(u64::from(l.x) + u64::from(l.width) <= u64::from(w));
            assert!(u64::from(l.y) + u64::from(l.height) <= u64::from(h));
            assert!(l.width != 0 && l.height != 0);
        }
    }
    // Every other request a client sends: read without a panic.
    let _ = capture::screenshot(body);
    let _ = capture::accessibility_get(body);
    let _ = place_keyed::scene_submit_layer(body);
    let _ = place_keyed::scene_remove_layer(body);
    let _ = remove::scene_remove(body);
    let _ = place_keyed::focus_set_window(body);
    let _ = pointer::focus_set(body);
    let _ = pointer::cursor_update(body);
    let _ = pointer::cursor_shape(body);
    let _ = present::present(body);
    let _ = present::present_layer(body);
    if let Some((shown, _, p)) = present::present_layer_parts(body) {
        within(&p, shown.area);
    }
    if let Some((c, p)) = layer_content_parts(body) {
        within(&p, c.area);
    }
    let _ = toggles::lid(body);
    let _ = toggles::idle_blank(body);
    let _ = toggles::frame_trace(body);
    let _ = toggles::brightness_step(body);
    let _ = toggles::output_layout(body);
    let _ = toggles::indicator(body);
    let _ = toggles::night_light(body);
    let _ = toggles::output_scale(body);
    let _ = vcp::set_vcp(body);
});
