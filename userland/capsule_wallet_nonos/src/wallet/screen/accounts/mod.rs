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
 * 10  ACCOUNTS: every account of this phrase in use here, the open one
 * marked. They are accounts 0 to 7 of the same recovery words, so any
 * Ethereum wallet restored from them shows the same addresses in the same
 * order. Opening one moves every screen, signature and shield to it.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use super::hits::{self, Press};
use super::page::{edges, lead};
use crate::wallet::accounts::file::MAX_ACCOUNTS;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::groups::shortened;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::state::State;

pub mod click;

const LEAD: &str = "Every account here comes from the same recovery words, numbered as \
     every Ethereum wallet numbers them. Open one to send, receive, stake and shield from it.";

fn hex(address: &[u8; 20]) -> String {
    let mut out = String::from("0x");
    for b in address {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

pub fn show(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::status::parts(state);
    let room = state.accounts.len() < MAX_ACCOUNTS as usize;
    let footer = [("Add account", Weight::Primary, room)];
    let spec = FrameSpec {
        number: "10",
        title: "Accounts",
        back: true,
        backdrop: Some(Backdrop::Settings),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    let labels: Vec<String> = state
        .accounts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let open = if i == state.account_open as usize { "  open" } else { "" };
            format!("Account {}  {}{open}", a.index, shortened(&hex(&a.address)))
        })
        .collect();
    let rows: Vec<(&str, bool)> = labels.iter().map(|t| (t.as_str(), true)).collect();
    let mut at = [Rect::new(0, 0, 0, 0); MAX_ACCOUNTS as usize];
    y += quiet_group(fb, c.x, y, c.w, &rows, &mut at[..rows.len()]);
    for (i, r) in at.iter().take(rows.len()).enumerate() {
        hits::put(Press::Row(i as u8), *r);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[room]);
}
