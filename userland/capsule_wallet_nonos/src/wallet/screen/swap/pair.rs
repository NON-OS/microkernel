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
 * The two sides of the trade: the token paid and the amount typed, then
 * the token bought and what the pool says comes back. The typed figure is
 * shown as the amount a contract would be given, not as raw digits.
 */

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{GAP, TIGHT};
use crate::wallet::screen::hits::Press;
use crate::wallet::screen::shield::parts::{chips, field, label};
use crate::wallet::state::State;
use crate::wallet::swap::{amount_text, count, token};

fn symbols() -> Vec<&'static str> {
    (0..count()).map(|i| token(i).symbol).collect()
}

fn text(state: &State, paying: bool) -> alloc::string::String {
    let mut b = [0u8; 48];
    let n = amount_text(state, paying, &mut b);
    alloc::string::String::from(core::str::from_utf8(&b[..n]).unwrap_or("0"))
}

pub fn pay(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let h = label(fb, c, y, "YOU PAY");
    let k = chips(fb, c, y + h, &symbols(), Some(state.swap_from), Press::Asset);
    let typed = if state.swap_in == 0 { alloc::string::String::new() } else { text(state, true) };
    let name = token(state.swap_from).symbol;
    let (_, f) = field(fb, c, y + h + k + TIGHT, name, &typed, "type an amount", true);
    h + k + TIGHT + f + GAP
}

pub fn receive(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let h = label(fb, c, y, "YOU RECEIVE");
    let k = chips(fb, c, y + h, &symbols(), Some(state.swap_to), Press::Pick);
    let got =
        if state.swap_quote.ready { text(state, false) } else { alloc::string::String::new() };
    let name = token(state.swap_to).symbol;
    let (_, f) = field(fb, c, y + h + k + TIGHT, name, &got, "no price yet", false);
    h + k + TIGHT + f + GAP
}
