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

//! The video mark drawn on posters and on the empty player.

use super::canvas::Sprite;
use super::shape::tri;
use super::unit::{blank, frame, W};

pub fn video(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    frame(&mut s, &m, [12, 24, 88, 76], m(W), rgb);
    tri(&mut s, [(m(42), m(38)), (m(42), m(62)), (m(64), m(50))], rgb);
    s
}
