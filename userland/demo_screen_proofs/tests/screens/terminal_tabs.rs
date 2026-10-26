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


//! The tab pills in the titlebar: an inactive tab is a pill as the active one
//! is, so its close mark sits inside a shape and not off its label in space.

use demo_screen_proofs::term::Terminal;
use nonos_app_skeleton::{App, InputEvent, InputKind, PaintBuffer, MOD_CTRL, MOD_SHIFT};

/// The pills' width and height (paint/tab_pill.rs); 4 px of gap end each pill.
const PILL_W: u32 = 160;
const PILL_H: u32 = 26;

#[test]
fn an_inactive_tab_is_a_visible_pill_around_its_close_mark() {
    nonos_libc::show(1920, 1080, "driver.virtio_gpu0");
    let mut t = Terminal::new();
    let (w, h) = (1240u32, 700u32);
    let mut body = vec![0u32; (w * h) as usize];
    t.paint(&mut PaintBuffer { pixels: &mut body, stride_words: w, width: w, height: h });
    let new_tab = InputEvent {
        kind: InputKind::KeyDown, flags: MOD_CTRL | MOD_SHIFT, code: 0x54,
        x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0,
    };
    t.on_event(new_tab);
    t.paint(&mut PaintBuffer { pixels: &mut body, stride_words: w, width: w, height: h });
    let aw = t.titlebar_accessory_w();
    assert!(aw >= 2 * PILL_W, "two pills fit: {aw}");
    let mut acc = vec![0u32; (aw * PILL_H) as usize];
    let pb = PaintBuffer { pixels: &mut acc, stride_words: aw, width: aw, height: PILL_H };
    t.paint_accessory(&mut { pb });
    let at = |x: u32, y: u32| acc[(y * aw + x) as usize];
    // Tab 0 is inactive now. Between its label and its close mark lies its
    // pill; the 4 px gap after it is the bar.
    let (inside, gap) = (at(PILL_W - 4 - 34, PILL_H / 2), at(PILL_W - 2, PILL_H / 2));
    assert_ne!(inside, gap, "the inactive pill shows: {inside:#x} against the bar {gap:#x}");
    let edge = at(PILL_W / 2, 0);
    assert_ne!(edge, gap, "and so does its edge");
    assert_ne!(edge, inside, "an edge of its own, not the fill: {edge:#x}");
}
