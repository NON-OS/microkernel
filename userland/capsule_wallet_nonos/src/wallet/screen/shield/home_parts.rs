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
 * The pieces of the Shield home: balances with the shielded rows first,
 * the round actions and the private receive address.
 */

use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;

use super::amounts::{shielded_eth, shielded_nox};
use crate::wallet::etna::parts::round::{round_action, round_height};
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::symbol::Symbol;
use crate::wallet::screen::amounts::{eth, nox};
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::shield_ui::{SHIELD_DEPOSIT, SHIELD_SEND, SHIELD_WITHDRAW};
use crate::wallet::state::State;

pub fn balances(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let rows: [(&str, String); 4] = [
        ("Shielded ETH", shielded_eth(state)),
        ("Shielded NOX", shielded_nox(state)),
        ("Public ETH", eth(state)),
        ("Public NOX", nox(state)),
    ];
    let at = Rect::new(c.x, y, c.w, row_height() * 4);
    tile(fb, at);
    for (i, (name, value)) in rows.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i == 3);
    }
    at.h
}

const ACTIONS: [(&str, Symbol, u8); 3] = [
    ("Deposit", Symbol::LockShield, SHIELD_DEPOSIT),
    ("Send", Symbol::ArrowUp, SHIELD_SEND),
    ("Withdraw", Symbol::ArrowDownToLine, SHIELD_WITHDRAW),
];

pub fn actions(fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let cell = c.w / ACTIONS.len() as u32;
    for (i, (title, sign, screen)) in ACTIONS.iter().enumerate() {
        let at = Rect::new(c.x + cell * i as u32, y, cell, round_height());
        round_action(fb, at, title, *sign, true);
        hits::put(Press::Go(*screen), at);
    }
    round_height()
}

pub fn receive(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    super::home_receive::receive(state, fb, c, y)
}
