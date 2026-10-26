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

//! A window framed as app_skeleton's runner frames it (paint_draw.rs): the
//! frame and titlebar at the display's quarters, then the app in the content
//! rectangle, at the size fit_rule opens it at on that display.

use nonos_app_skeleton::runner::chrome::quarters_for;
use nonos_app_skeleton::runner::fit_rule::fit_manifest;
use nonos_app_skeleton::{App, AppManifest, PaintBuffer};
use nonos_toolkit::decorations::{content_rect_at, draw_frame_at, DecorationHit, FrameState};

pub struct Shot {
    pub at: AppManifest,
    pub pixels: Vec<u32>,
}

/// Open `app` on a `w` x `h` display and paint one frame of it.
pub fn shoot<A: App>(app: &mut A, w: u32, h: u32) -> Shot {
    let at = fit_manifest(app.manifest(), w, h);
    let q = quarters_for(w, h);
    let mut pixels = vec![0u32; (at.width * at.height) as usize];
    let mut fb = PaintBuffer {
        pixels: &mut pixels,
        stride_words: at.width,
        width: at.width,
        height: at.height,
    };
    let state = FrameState { hover: DecorationHit::None, focused: true };
    draw_frame_at(&mut fb, false, at.title, state, 0, q);
    let c = content_rect_at(at.width, at.height, false, q);
    let framed = fb.pixels.to_vec();
    app.paint(&mut fb.sub(c.x, c.y, c.w, c.h));
    // The app owns its content rectangle and nothing else: the frame, its
    // border and its shadow stay as the frame drew them.
    for (i, (now, was)) in pixels.iter().zip(&framed).enumerate() {
        let (x, y) = (i as u32 % at.width, i as u32 / at.width);
        let inside = (c.x..c.x + c.w).contains(&x) && (c.y..c.y + c.h).contains(&y);
        assert!(inside || now == was, "{}: painted outside its content at {x},{y}", w * 10_000 + h);
    }
    Shot { at, pixels }
}
