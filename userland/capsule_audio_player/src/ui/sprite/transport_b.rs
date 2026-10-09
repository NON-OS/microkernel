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

//! Stroke transport glyphs: shuffle and repeat.

use super::canvas::Sprite;
use super::{shape, stroke};

pub fn shuffle(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(8);
    // Two paths that cross and leave to the right, each with its head: the
    // shape every player uses, where a bare cross read as "close".
    stroke::line(&mut s, (m(14), m(30)), (m(36), m(30)), t, rgb);
    stroke::line(&mut s, (m(36), m(30)), (m(62), m(70)), t, rgb);
    stroke::line(&mut s, (m(62), m(70)), (m(74), m(70)), t, rgb);
    stroke::line(&mut s, (m(14), m(70)), (m(36), m(70)), t, rgb);
    stroke::line(&mut s, (m(36), m(70)), (m(62), m(30)), t, rgb);
    stroke::line(&mut s, (m(62), m(30)), (m(74), m(30)), t, rgb);
    shape::tri(&mut s, [(m(72), m(18)), (m(90), m(30)), (m(72), m(42))], rgb);
    shape::tri(&mut s, [(m(72), m(58)), (m(90), m(70)), (m(72), m(82))], rgb);
    s
}

pub fn repeat(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(8);
    // A loop of two halves, each ending in a head, so it reads as going
    // round rather than as a box.
    stroke::line(&mut s, (m(20), m(60)), (m(20), m(32)), t, rgb);
    stroke::line(&mut s, (m(20), m(32)), (m(66), m(32)), t, rgb);
    shape::tri(&mut s, [(m(64), m(20)), (m(84), m(32)), (m(64), m(44))], rgb);
    stroke::line(&mut s, (m(80), m(40)), (m(80), m(68)), t, rgb);
    stroke::line(&mut s, (m(80), m(68)), (m(34), m(68)), t, rgb);
    shape::tri(&mut s, [(m(36), m(56)), (m(16), m(68)), (m(36), m(80))], rgb);
    s
}
