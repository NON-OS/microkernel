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

//! Sidebar destinations: library and folders.

use super::canvas::Sprite;
use super::unit::{blank, frame, path, W};

pub fn library(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    frame(&mut s, &m, [16, 16, 42, 42], t, rgb);
    frame(&mut s, &m, [58, 16, 84, 42], t, rgb);
    frame(&mut s, &m, [16, 58, 42, 84], t, rgb);
    frame(&mut s, &m, [58, 58, 84, 84], t, rgb);
    s
}

pub fn files(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    path(&mut s, &m, &[(14, 78), (14, 26), (38, 26), (48, 40), (86, 40), (86, 78), (14, 78)], t, rgb);
    s
}
