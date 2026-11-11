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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text;

use crate::store::state::State;

use super::bytes::as_str;
use super::card;
use super::metrics::metrics;

/// How far below the list's top an empty list's words start, in logical
/// pixels.
const EMPTY_TOP: u32 = 10;

pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, rows: usize) {
    let (visible, m) = (state.visible(), metrics());
    if visible.is_empty() {
        let (step, ink) = (line_height(m.body_px).max(1), roles().muted);
        let first = (y + scale().px(EMPTY_TOP)) as i32;
        for (i, said) in empty_because(state).split(|&b| b == b'\n').enumerate() {
            text::left(fb, x, first + i as i32 * step, as_str(said), ink, m.body_px);
        }
        return;
    }
    for slot in 0..rows {
        let Some(&index) = visible.get(state.scroll + slot) else { break };
        let Some(listing) = state.listings.get(index) else { break };
        let top = y + slot as u32 * (m.card_h + m.card_gap);
        let fetch = state.fetch_of(listing);
        card::paint(fb, listing, fetch, (x, top, w), (state, state.scroll + slot));
    }
}

/// Why there is nothing to show, one line per `\n`.
fn empty_because(state: &State) -> &[u8] {
    match (state.trouble.as_deref(), state.listings.is_empty()) {
        (Some(why), _) => why,
        (None, true) if !state.loaded => b"asking the market for its catalogue",
        (None, true) => b"the catalogue is empty",
        (None, false) if !state.search.text().is_empty() => b"nothing matches that",
        (None, false) => state.tab.empty(),
    }
}
