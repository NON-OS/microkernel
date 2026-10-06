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

use super::frame_rect::{frame_rect_at, light_rect_at, titlebar_rect_at};
use super::metrics::LIGHT_HIT_PAD;
use super::scale::{at, ONE};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DecorationHit {
    None,
    Titlebar,
    CloseButton,
    MinimizeButton,
    MaximizeButton,
}

const LIGHT_HITS: [DecorationHit; 3] =
    [DecorationHit::CloseButton, DecorationHit::MinimizeButton, DecorationHit::MaximizeButton];

pub fn hit_test(w: u32, h: u32, maximized: bool, x: u32, y: u32) -> DecorationHit {
    hit_test_at(w, h, maximized, x, y, ONE)
}

/// What a press at `x`, `y` lands on, with the frame drawn at `quarters` of
/// display scale. The buttons' targets grow with the buttons.
pub fn hit_test_at(
    w: u32,
    h: u32,
    maximized: bool,
    x: u32,
    y: u32,
    quarters: u32,
) -> DecorationHit {
    if !frame_rect_at(w, h, maximized, quarters).contains(x, y) {
        return DecorationHit::None;
    }
    if !titlebar_rect_at(w, h, maximized, quarters).contains(x, y) {
        return DecorationHit::None;
    }
    let pad = at(LIGHT_HIT_PAD, quarters);
    for (i, hit) in LIGHT_HITS.iter().enumerate() {
        if light_rect_at(i as u32, w, h, maximized, quarters).inflate(pad).contains(x, y) {
            return *hit;
        }
    }
    DecorationHit::Titlebar
}
