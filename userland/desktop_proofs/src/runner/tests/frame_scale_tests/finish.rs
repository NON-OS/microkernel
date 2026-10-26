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
//! A window's frame at a scale: its measures, its pixels and its title's rows.
//! The runtime's last pass strokes the frame's edge again after the app has
//! painted. The focused window's rule is drawn over that stroke, so it is
//! still there on the frame a person sees; an unfocused window has none.

// The runtime's pass itself, mounted here: it names its scale as
// `super::chrome`, which this module provides.
#[path = "../../../../../app_skeleton/src/runner/frame_finish.rs"]
mod frame_finish;
use frame_finish::finish;
use nonos_app_skeleton::runner::chrome;

use super::rig::{AT_REST, H, W};
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::{draw_frame_at, focus_rule, frame_rect_at, FrameState};

fn finished(focused: bool) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw_frame_at(&mut fb, false, b"Files", FrameState { focused, ..AT_REST }, 0, 4);
    finish(&mut fb, false, focused);
    px
}

#[test]
fn the_focus_rule_outlasts_the_last_pass() {
    let f = frame_rect_at(W, H, false, 4);
    let top = |px: &[u32]| px[(f.y * W + W / 2) as usize];
    assert_eq!(top(&finished(true)), focus_rule(), "the focused window keeps its rule");
    assert_ne!(top(&finished(false)), focus_rule(), "the other has none");
}
