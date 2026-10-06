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

use super::damage::Damage;
use super::geometry::{bubble_band, page_rect, pill_rect, search_rect, toolbar_rect, Rect};

/* What changed, in the terms the event and tick code speak. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    OmniboxEdit,
    /* On the home page the search bar mirrors the address text. */
    HomeEdit,
    Toolbar,
    Page,
    Scroll,
    Bubble,
    Full,
}

pub fn bits(c: Change) -> Damage {
    match c {
        Change::OmniboxEdit => Damage::PILL,
        Change::HomeEdit => Damage(Damage::PILL.0 | Damage::HOME_BAR.0),
        Change::Toolbar => Damage::TOOLBAR,
        Change::Page => Damage::PAGE,
        Change::Scroll => Damage::SCROLL,
        Change::Bubble => Damage::BUBBLE,
        Change::Full => Damage::FULL,
    }
}

/* The rect covering every dirtied part, or None when the whole window has
 * to be drawn: a full change, or nothing recorded to go by. */
pub fn rect_of(d: Damage, width: u32, height: u32) -> Option<Rect> {
    if d.is_empty() || d.has(Damage::FULL) {
        return None;
    }
    let parts = [
        (Damage::PILL, pill_rect(width)),
        (Damage::TOOLBAR, toolbar_rect(width)),
        (Damage::HOME_BAR, search_rect(width)),
        (Damage(Damage::PAGE.0 | Damage::SCROLL.0), page_rect(width, height)),
        (Damage::BUBBLE, bubble_band(width, height)),
    ];
    parts.iter().filter(|(b, _)| d.has(*b)).map(|(_, r)| *r).reduce(Rect::union)
}

#[cfg(test)]
pub fn damage_for(c: Change, width: u32, height: u32) -> Option<Rect> {
    rect_of(bits(c), width, height)
}
