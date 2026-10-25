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

//! The home screen's tiles: the coins, where their figures came from, and
//! the quiet rows below.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::amounts::{eth, nox, usdc};
use super::hits::{self, Press};
use super::status_words::unread_balances;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::state::State;

/// The coins the account holds, in one tile; one line in their place while
/// none of them has been read. Its height.
pub(super) fn coins(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let none = !state.reads.balance_ready && !state.nox.balance_ready && !state.reads.usdc_ready;
    let rows: Vec<(&str, String)> = if none {
        let refreshing = state.net.job.is_some();
        let said = unread_balances(refreshing, state.net.link.rpc_connect_ok);
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
pub(super) fn read(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let last = state.net.last_read[usize::from(crate::wallet::chain::is_sepolia())];
    let line = crate::wallet::net::last_read::read_line(last, nonos_libc::mk_uptime_ms());
    crate::wallet::etna::parts::fact::fact(fb, c.x, y + GAP / 2, c.w, "read", &line) + GAP / 2
}

/// Staking and settings, as quiet rows. Their height.
pub(super) fn more(fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let rows = [("Stake NOX", true), ("Settings and network", true)];
    let mut at = [Rect::new(0, 0, 0, 0); 2];
    let h = crate::wallet::etna::parts::quiet::quiet_group(fb, c.x, y, c.w, &rows, &mut at);
    hits::put(Press::Row(0), at[0]);
    hits::put(Press::Row(1), at[1]);
    h
}
