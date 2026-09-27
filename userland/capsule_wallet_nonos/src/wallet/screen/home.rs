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

//! 03  WALLET, the home screen, as Overview.swift lays it out: the mark and
//! the way to settings, the account and network, the coins the account
//! holds, and the four things a holder does.

use nonos_app_skeleton::PaintBuffer;

use super::amounts::{eth, nox, UNREAD};
use super::hits::{self, Press};
use super::home_pills::pills;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BACK, GAP};
use crate::wallet::state::State;

pub fn home(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "03",
        title: "Wallet",
        back: false,
        backdrop: Some(Backdrop::Home),
        failure: None,
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    crate::wallet::paint::logo::logo(fb, c.x, c.y + (BACK - 18) / 2, 18);
    let gear = Rect::new(c.x + c.w - BACK, c.y, BACK, BACK);
    crate::wallet::etna::symbol::gear(fb, gear);
    hits::put(Press::Settings, gear);
    let mut y = c.y + BACK + GAP;
    y += pills(state, fb, c, y) + GAP;
    let rows =
        [("ETH", eth(state)), ("NOX", nox(state)), ("USDC", alloc::string::String::from(UNREAD))];
    let th = row_height() * rows.len() as u32;
    let at = Rect::new(c.x, y, c.w, th);
    tile(fb, at);
    for (i, (name, value)) in rows.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == rows.len());
    }
    y += th + GAP;
    super::home_actions::actions(state, fb, c, y);
    hits::reach(
        y + crate::wallet::etna::parts::round::round_height(),
        state.scroll,
        l.content_bottom,
    );
    end(fb, &spec, &mut l);
}
