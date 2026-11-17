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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Pill, Rect as Area, Tone};

use crate::browser::ui::rect::Rect;

const BADGE_INSET: u32 = 20;
const BADGE: &str = "History clears at shutdown";

/// The session badge in the hero's top right corner, a shield and its words
/// in the Ok tone. It stays clear of the greeting ending at `greet_right`:
/// when they would meet, it is left out.
pub fn paint(fb: &mut PaintBuffer, hero: Rect, greet_right: i32) {
    let pill = Pill::new(BADGE).icon(IconId::Shield).tone(Tone::Ok);
    let ((w, h), inset) = (pill.size(), scale().px(BADGE_INSET) as i32);
    let x = hero.right() - inset - w as i32;
    let y = hero.y + inset;
    if x < greet_right + inset || y < 0 {
        return;
    }
    pill.draw(fb, Area { x: x as u32, y: y as u32, w, h });
}
