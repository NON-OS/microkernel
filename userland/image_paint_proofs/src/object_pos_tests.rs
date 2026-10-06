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

/* object-position: a covering image crops toward the named edge, and a
 * contained one sits there in its letterbox. */
use crate::blit_img::{frame, FW};
use crate::browser::css::ObjectFit;
use crate::browser::image::{blit_rect, Decoded};

const RED: u32 = 0xffff_0000;
const BLUE: u32 = 0xff00_00ff;

/* A 10 x 20 image, its top half red and its bottom half blue. */
fn halves() -> Decoded {
    let px = (0..200).map(|i| if i < 100 { RED } else { BLUE }).collect();
    Decoded { w: 10, h: 20, px }
}

fn at(fit: ObjectFit, x: usize, y: usize) -> u32 {
    frame(|fb| blit_rect(fb, &halves(), [0, 0, 10, 10], fit, 255, None))[y * FW as usize + x]
}

#[test]
fn cover_crops_toward_the_position() {
    let top = ObjectFit::COVER.at([(0, 0), (0, 0)]);
    let bottom = ObjectFit::COVER.at([(0, 1000), (0, 1000)]);
    assert!((0..10).all(|y| at(top, 5, y) == RED), "0% shows the top half");
    assert!((0..10).all(|y| at(bottom, 5, y) == BLUE), "100% shows the bottom half");
    assert_eq!((at(ObjectFit::COVER, 5, 0), at(ObjectFit::COVER, 5, 9)), (RED, BLUE), "centred");
}

#[test]
fn contain_sits_at_the_position() {
    /* 10 x 20 contained in 10 x 10 is 5 x 10: 5 px of room across. */
    let left = ObjectFit::CONTAIN.at([(0, 0), (0, 500)]);
    let right = ObjectFit::CONTAIN.at([(0, 1000), (0, 500)]);
    assert_eq!((at(left, 0, 0), at(right, 9, 0)), (RED, RED));
    assert_ne!(at(left, 9, 0), RED, "the right side is letterbox");
    assert_ne!(at(right, 0, 0), RED, "the left side is letterbox");
}
