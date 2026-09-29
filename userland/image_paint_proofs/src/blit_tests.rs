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

//! The clipped blit is bit-identical to the old full-box loop wherever a
//! pixel shows, for every fit, scale, opacity and clip.
use crate::blit_img::{frame, image};
use crate::blit_ref;
use crate::browser::css::ObjectFit;
use crate::browser::image::blit_into;

#[test]
fn scaled_and_clipped_blits_match_the_full_box_loop() {
    let clips = [None, Some([5, 3, 40, 50]), Some([-9, -9, 500, 500]), Some([30, 30, 31, 31])];
    for (w, h) in [(1, 1), (7, 5), (40, 90), (97, 61)] {
        let img = image(w, h);
        for dest in [[3, 2, 50, 40], [0, 0, 97, 61], [10, 20, 300, 7], [60, 50, 80, 80]] {
            for (alpha, clip) in
                [(255u8, clips[0]), (200, clips[1]), (255, clips[2]), (90, clips[3])]
            {
                let src = [0, 0, w, h];
                let want = frame(|fb| blit_ref::draw(fb, &img, dest, src, alpha, clip));
                let got = frame(|fb| blit_into(fb, &img, dest, ObjectFit::Fill, alpha, clip));
                assert_eq!(got, want, "img {w}x{h} dest {dest:?} clip {clip:?}");
            }
        }
    }
}
