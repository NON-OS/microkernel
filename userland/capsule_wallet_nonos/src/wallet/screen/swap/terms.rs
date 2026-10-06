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

/*
 * Every term of the trade, open rather than folded away: the rate, how far
 * the trade moves the price, the minimum the chain will enforce, slippage,
 * the network fee and the route. A price impact large enough to hurt is
 * written out as a warning under the tile, in words.
 */

use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BAD, GAP, TEXT_3};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
use crate::wallet::state::State;
use crate::wallet::swap::{self, token};

const HURTS: &str = "This trade moves the price a long way against itself. A smaller \
     amount would get a better rate.";

fn s(f: impl Fn(&mut [u8]) -> usize) -> String {
    let mut b = [0u8; 64];
    let n = f(&mut b);
    String::from(core::str::from_utf8(&b[..n]).unwrap_or(""))
}

pub fn terms(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let q = state.swap_quote;
    if !q.ready {
        return 0;
    }
    let (from, to) = (token(state.swap_from), token(state.swap_to));
    let rows: Vec<(&str, String)> = Vec::from([
        ("Rate", s(|b| swap::rate_text(state, b))),
        ("Price impact", s(|b| swap::bps_text(q.impact_bps, b))),
        ("Minimum received", s(|b| swap::min_out_text(state, b))),
        ("Slippage", s(|b| swap::slippage_text(state, b))),
        ("Network fee", s(|b| swap::gas_text(q.gas, b))),
        ("Route", s(|b| swap::route_text(from, to, b))),
    ]);
    let at = Rect::new(c.x, y, c.w, row_height() * rows.len() as u32);
    tile(fb, at);
    for (i, (name, value)) in rows.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == rows.len());
    }
    let mut h = at.h + GAP;
    if swap::is_warning(q.impact_bps) || swap::is_dangerous(q.impact_bps) {
        let ink = if swap::is_dangerous(q.impact_bps) { BAD } else { TEXT_3 };
        h += wrapped(fb, c.x as i32, (y + h) as i32, c.w as i32, Role::Lead, HURTS, ink) as u32;
    }
    h
}
