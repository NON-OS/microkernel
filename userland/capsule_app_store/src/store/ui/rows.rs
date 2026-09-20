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
//! The catalogue, as a column of cards.

use nonos_app_skeleton::PaintBuffer;

use crate::store::state::State;
use crate::store::theme::MUTED;

use super::card;
use super::metrics::{BODY_PX, CARD_GAP, CARD_H};
use super::text;

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, rows: usize) {
    let visible = state.visible();
    if visible.is_empty() {
        text::line(fb, x, y as i32 + 10, empty_because(state), MUTED, BODY_PX);
        return;
    }
    for slot in 0..rows {
        let Some(&index) = visible.get(state.scroll + slot) else { break };
        let Some(listing) = state.listings.get(index) else { break };
        let top = y + slot as u32 * (CARD_H + CARD_GAP);
        card::paint(fb, listing, x, top, w, state.scroll + slot == state.cursor);
    }
}

/// Why there is nothing to show.
fn empty_because(state: &State) -> &'static [u8] {
    match (state.trouble, state.listings.is_empty()) {
        (Some(why), _) => why,
        (None, true) => b"the catalogue is empty",
        (None, false) if !state.search.text().is_empty() => b"nothing matches that",
        (None, false) => b"nothing under this tab",
    }
}
