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

//! The fields the Bond's and a release's calls are typed into, and their
//! rows (acts_rows.rs), drawn under the entry's binding and attests.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_marketplace_abi::MarketplaceEntry;

use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::publish::{FOCUS_AGENT, FOCUS_AMOUNT, FOCUS_URI};
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::send::amount_text;
use crate::wallet::state::State;

pub fn acts(state: &State, e: &MarketplaceEntry, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let p = &state.publisher;
    let amount = amount_text(&p.amount);
    let fields = [
        (FOCUS_AMOUNT, "BOND AMOUNT (NOX)", amount.as_str(), "0.0"),
        (FOCUS_AGENT, "DELIVERY AGENT", p.agent.as_str(), "0x and 40 hex digits"),
        (FOCUS_URI, "RELEASE URI (https)", p.uri.as_str(), "https://"),
    ];
    let mut h = GAP;
    for (n, name, text, hint) in fields {
        let (at, used) = field(fb, c, y + h, name, text, hint, p.focus == n);
        hits::put(Press::Field(n), at);
        h += used + GAP / 2;
    }
    let rows = super::acts_rows::rows(state, e);
    let list: Vec<(&str, bool)> = rows.iter().map(|(t, on, _)| (t.as_str(), *on)).collect();
    let mut at = alloc::vec![Rect::new(0, 0, 0, 0); list.len()];
    h += GAP + quiet_group(fb, c.x, y + h + GAP, c.w, &list, &mut at);
    for ((_, on, r), a) in rows.iter().zip(&at) {
        if *on {
            hits::put(Press::Row(*r), *a);
        }
    }
    h
}
