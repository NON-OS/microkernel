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

//! The settings screen's network chips and the facts of this account.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::chain;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line};
use crate::wallet::etna::tokens::{GAP, TEXT_3, TIGHT};
use crate::wallet::etna::Role;
use crate::wallet::screen::hits::Press;
use crate::wallet::screen::shield::parts::chips;
use crate::wallet::state::State;

fn heading(fb: &mut PaintBuffer, c: Rect, y: u32, text: &str) -> u32 {
    draw_in(fb, c.x as i32, y as i32, Role::Fact, text, TEXT_3);
    line(Role::Fact) as u32 + TIGHT
}

/// The network chips and the facts under them, from `y`; where they end.
pub(super) fn facts(state: &State, fb: &mut PaintBuffer, c: Rect, mut y: u32) -> u32 {
    y += heading(fb, c, y, "NETWORK");
    let picked = Some(chain::is_sepolia() as u8);
    y += chips(fb, c, y, &["Ethereum mainnet", "Sepolia"], picked, Press::Network) + GAP;
    let address = crate::wallet::screen::receive_address::address_text(state);
    let shown = if state.address_ready {
        crate::wallet::etna::groups::shortened(&address)
    } else {
        alloc::string::String::from("none yet")
    };
    y += fact(fb, c.x, y, c.w, "account", &shown);
    let key = if state.vault_saved { "sealed to this machine" } else { "RAM only, gone at reboot" };
    y += fact(fb, c.x, y, c.w, "key", key);
    y += fact(
        fb,
        c.x,
        y,
        c.w,
        "requests go over",
        crate::wallet::net::route_value(state.net.link.route),
    );
    y += fact(fb, c.x, y, c.w, "RPC host", chain::rpc_host());
    let now = nonos_libc::mk_uptime_ms();
    let line = crate::wallet::net::last_read::read_line;
    y += fact(fb, c.x, y, c.w, "mainnet, last read", &line(state.net.last_read[0], now));
    y += fact(fb, c.x, y, c.w, "Sepolia, last read", &line(state.net.last_read[1], now));
    y += fact(fb, c.x, y, c.w, "chain id", &alloc::format!("{}", chain::current().id)) + GAP;
    y
}
