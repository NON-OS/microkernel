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
//! The audio player's ground: its accent glows show in the house theme, and in
//! every scheme every pixel still carries text and muted text at the body floor,
//! a scheme with no contrast to spare taking a fainter glow or none.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::{roles, store, theme_of, SCHEMES};

use crate::audio_ui::shell::ground::Ground;

const W: u32 = 800;
const H: u32 = 600;

fn ground() -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    Ground::build(W as i32, H as i32).blit(&mut fb);
    px
}

#[test]
fn the_glows_show_and_text_reads_on_every_pixel_of_the_ground() {
    let schemes = (0..SCHEMES.len() as u8).map(|i| (i, false)).chain([(0, true)]);
    for (i, hc) in schemes {
        store::replace(theme_of(i, hc));
        let r = roles();
        let px = ground();
        let off_grid = px[(W + W / 2 + 1) as usize];
        let shows = off_grid != r.bg.argb();
        assert!(shows || (i, hc) != (0, false), "the house theme shows no glow");
        for (n, &c) in px.iter().enumerate().step_by(7) {
            for (name, ink) in [("text", r.text), ("muted", r.muted)] {
                let at = (n as u32 % W, n as u32 / W);
                assert!(at_least(ink.argb(), c, BODY), "scheme {i} hc {hc}: {name} at {at:?}");
            }
        }
    }
    store::replace(theme_of(0, false));
}
