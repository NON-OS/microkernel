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

extern crate alloc;

use alloc::vec::Vec;

use super::layout::{CARD_H, FOOTER_H, HEADER_H, SECTION_GAP};
use super::recents_group::rel_time;
use super::screen_list::Line;
use super::screen_row::LABEL_ADV;
use super::state::State;

// How many journal entries the Continue Working column offers before the
// Storage section needs the remaining height.
const RECENT_MAX: usize = 3;
const GREET_ADV: u32 = 46;

/// Top of the CONTINUE WORKING section label.
pub fn cards_label_y() -> u32 {
    HEADER_H + 16 + GREET_ADV + SECTION_GAP
}

/// The journal cards Home offers, capped both by `RECENT_MAX` and by the height
/// left above the footer. The painter draws these and `screen_hit` searches
/// them, so a card opens the path it is showing.
pub fn home_lines(state: &State, now: u64) -> Vec<Line> {
    let bottom = state.win_h.saturating_sub(FOOTER_H);
    let mut y = cards_label_y() + LABEL_ADV;
    let mut out = Vec::new();
    for (ms, path) in state.recents.iter().take(RECENT_MAX) {
        if y + CARD_H > bottom {
            break;
        }
        out.push(Line::row(y, CARD_H, path, rel_time(now, *ms), path.ends_with('/')));
        y += CARD_H;
    }
    out
}

/// The y just past the card column, where the Storage section starts.
pub fn cards_bottom(state: &State, now: u64) -> u32 {
    let empty_adv = cards_label_y() + LABEL_ADV * 2;
    home_lines(state, now).last().map(|l| l.y + l.h).unwrap_or(empty_adv) + SECTION_GAP
}
