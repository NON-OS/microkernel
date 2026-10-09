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

//! The home screen, as Overview.swift lays it out: the mark and the way to
//! settings, the account and network, the coins the account holds, and the
//! things a holder does. In a wide window it is two columns.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::amounts::{eth, nox, usdc};
use super::hits::{self, Press};
use super::home_pills::pills;
use super::status_words::unread_balances;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::layout::halves;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::{BACK, GAP, TEXT_2};
use crate::wallet::etna::wrap::wrapped;
use crate::wallet::etna::Role;
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
    let mut top = c.y + BACK + GAP;
    /* A wallet this machine does not keep says so first, in full width. */
    if let Some(note) = state.kept_note {
        let text = super::status_words::capital(note);
        top +=
            wrapped(fb, c.x as i32, top as i32, c.w as i32, Role::Lead, &text, TEXT_2) as u32 + GAP;
    }
    /* Wide: the coins and what to do with them on the left, the account,
     * its network and the rest on the right. Narrow: one column. */
    let y = if crate::wallet::etna::current::now().two_up {
        let ((lx, lw), (rx, rw)) = halves(c.x, c.w, 2 * GAP);
        let left = Rect::new(lx, top, lw, c.h);
        let right = Rect::new(rx, top, rw, c.h);
        let mut ly = top + coins(state, fb, left, top);
        ly += read(state, fb, left, ly) + GAP;
        ly += super::home_actions::actions(state, fb, left, ly);
        let mut ry = top + pills(state, fb, right, top) + GAP;
        ry += more(fb, right, ry);
        ly.max(ry)
    } else {
        let mut y = top;
        y += pills(state, fb, c, y) + GAP;
        y += coins(state, fb, c, y);
        y += read(state, fb, c, y) + GAP;
        y += super::home_actions::actions(state, fb, c, y) + GAP;
        y + more(fb, c, y)
    };
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
}

/// The coins the account holds, in one tile; one line in their place while
/// none of them has been read. Its height.
fn coins(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let none = !state.balance_ready && !state.nox.balance_ready && !state.usdc_ready;
    let rows: Vec<(&str, String)> = if none {
        let refreshing = state.net_job.is_some();
        let said = unread_balances(refreshing, state.net.rpc_connect_ok);
        Vec::from([("Balances", String::from(said))])
    } else {
        Vec::from([("ETH", eth(state)), ("NOX", nox(state)), ("USDC", usdc(state))])
    };
    let th = row_height() * rows.len() as u32;
    let at = Rect::new(c.x, y, c.w, th);
    tile(fb, at);
    for (i, (name, value)) in rows.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == rows.len());
    }
    th
}

/// Where the figures above came from: the block, the host and the network
/// of this network's last whole read, and how long ago. Its height.
fn read(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let last = state.last_read[usize::from(crate::wallet::chain::is_sepolia())];
    let line = crate::wallet::net::last_read::read_line(last, nonos_libc::mk_uptime_ms());
    crate::wallet::etna::parts::fact::fact(fb, c.x, y + GAP / 2, c.w, "read", &line) + GAP / 2
}

/// Staking and settings, as quiet rows. Their height.
fn more(fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let rows = [("Stake NOX", true), ("Settings and network", true)];
    let mut at = [Rect::new(0, 0, 0, 0); 2];
    let h = crate::wallet::etna::parts::quiet::quiet_group(fb, c.x, y, c.w, &rows, &mut at);
    hits::put(Press::Row(0), at[0]);
    hits::put(Press::Row(1), at[1]);
    h
}
