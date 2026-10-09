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

//! The account the home screen is about, cut to its ends so it can be
//! checked, and the network it is on: two outlined pills on one line.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use crate::wallet::etna::groups::shortened;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw, line, width};
use crate::wallet::etna::tokens::{CORNER, HALF, OUTLINE};
use crate::wallet::etna::Role;
use crate::wallet::state::State;

const PILL_H: u32 = 36;

fn address(state: &State) -> String {
    if !state.address_ready {
        return String::from("No account yet");
    }
    let mut hex = String::from("0x");
    for b in state.address {
        hex.push_str(&alloc::format!("{b:02x}"));
    }
    /* With more than one account, say which is open. */
    match state.accounts.get(state.account_open as usize) {
        Some(a) if state.accounts.len() > 1 => alloc::format!("{}  {}", a.index, shortened(&hex)),
        _ => shortened(&hex),
    }
}

fn pill(fb: &mut PaintBuffer, x: u32, y: u32, text: &str) -> Rect {
    let w = width(Role::RowValue, text) as u32 + 2 * HALF;
    fb.stroke_round(x, y, w, PILL_H, CORNER, 1, OUTLINE);
    draw(
        fb,
        (x + HALF) as i32,
        (y + (PILL_H - line(Role::RowValue) as u32) / 2) as i32,
        Role::RowValue,
        text,
    );
    Rect::new(x, y, w, PILL_H)
}

/// Draw both pills on the line at `y`; returns their height.
pub fn pills(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let account = pill(fb, c.x, y, &address(state));
    hits::put(Press::Accounts, account);
    let net = crate::wallet::chain::current().name;
    let nw = width(Role::RowValue, net) as u32 + 2 * HALF;
    let at = pill(fb, c.x + c.w - nw, y, net);
    hits::put(Press::Network(0), at);
    PILL_H
}
