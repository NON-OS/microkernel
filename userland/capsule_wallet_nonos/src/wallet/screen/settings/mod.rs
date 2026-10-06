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
//! 09  SETTINGS: the network this account is on, where its key is kept,
//! what the requests go over, and the things done to the key itself.

use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line};
use crate::wallet::etna::tokens::{GAP, TEXT_3, TIGHT};
use crate::wallet::etna::Role;
use crate::wallet::screen::shield::parts::chips;
use crate::wallet::state::State;

pub mod click;

const LEAD: &str = "One key holds an account on Ethereum mainnet and on Sepolia. \
     Sepolia is the test network, where the shield pool runs and nothing has value.";

/// The rows of the key list, in press order.
pub const ROWS: [&str; 4] = [
    "Show the private key",
    "Restore from recovery words",
    "Import a private key",
    "Lock the screen",
];

fn heading(fb: &mut PaintBuffer, c: Rect, y: u32, text: &str) -> u32 {
    draw_in(fb, c.x as i32, y as i32, Role::Fact, text, TEXT_3);
    line(Role::Fact) as u32 + TIGHT
}

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let spec = FrameSpec {
        number: "09",
        title: "Settings",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &[],
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    y += heading(fb, c, y, "NETWORK");
    let picked = Some(chain::is_sepolia() as u8);
    y += chips(fb, c, y, &["Ethereum mainnet", "Sepolia"], picked, Press::Network) + GAP;
    let address = super::receive_address::address_text(state);
    let shown = if state.address_ready {
        crate::wallet::etna::groups::shortened(&address)
    } else {
        alloc::string::String::from("none yet")
    };
    y += fact(fb, c.x, y, c.w, "account", &shown);
    let key = if state.vault_saved { "sealed to this machine" } else { "RAM only, gone at reboot" };
    y += fact(fb, c.x, y, c.w, "key", key);
    y +=
        fact(fb, c.x, y, c.w, "requests go over", crate::wallet::net::route_value(state.net.route));
    y += fact(fb, c.x, y, c.w, "RPC host", chain::rpc_host());
    let now = nonos_libc::mk_uptime_ms();
    let line = crate::wallet::net::last_read::read_line;
    y += fact(fb, c.x, y, c.w, "mainnet, last read", &line(state.last_read[0], now));
    y += fact(fb, c.x, y, c.w, "Sepolia, last read", &line(state.last_read[1], now));
    y += fact(fb, c.x, y, c.w, "chain id", &alloc::format!("{}", chain::current().id)) + GAP;
    let rows: [(&str, bool); 4] = [
        (ROWS[0], state.address_ready),
        (ROWS[1], true),
        (ROWS[2], true),
        (ROWS[3], state.address_ready),
    ];
    let mut at = [Rect::new(0, 0, 0, 0); 4];
    y += quiet_group(fb, c.x, y, c.w, &rows, &mut at);
    for (i, r) in at.iter().enumerate() {
        if rows[i].1 {
            hits::put(Press::Row(i as u8), *r);
        }
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[]);
}
