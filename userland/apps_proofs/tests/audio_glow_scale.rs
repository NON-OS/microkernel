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

//! The audio player's ground at every scale: its accent glows show in the
//! house theme, and in every scheme every pixel still carries text and muted
//! text at the body floor, a scheme with no contrast to spare taking a fainter
//! glow or none.

use apps_proofs::audio_ui::shell::ground::Ground;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::{roles, store, theme_of, SCHEMES};

fn ground(w: u32, h: u32) -> Vec<u32> {
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    Ground::build(w as i32, h as i32).blit(&mut fb);
    px
}

#[test]
fn the_glows_show_and_text_reads_on_every_pixel_of_the_ground_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h, hair) = (scale().px(800), scale().px(600), scale().hair());
        let schemes = (0..SCHEMES.len() as u8).map(|i| (i, false)).chain([(0, true)]);
        for (i, hc) in schemes {
            store::replace(theme_of(i, hc));
            let r = roles();
            let px = ground(w, h);
            let off_grid = px[(hair * w + w / 2 + hair) as usize];
            let shows = off_grid != r.bg.argb();
            assert!(shows || (i, hc) != (0, false), "q={q}: the house theme shows no glow");
            for (n, &c) in px.iter().enumerate().step_by(7) {
                for (name, ink) in [("text", r.text), ("muted", r.muted)] {
                    let at = (n as u32 % w, n as u32 / w);
                    let reads = at_least(ink.argb(), c, BODY);
                    assert!(reads, "q={q}: scheme {i} hc {hc}: {name} at {at:?}");
                }
            }
        }
        store::replace(theme_of(0, false));
    }
    set_scale(4);
}
