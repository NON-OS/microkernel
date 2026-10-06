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

//! 07  The staking review: what the transaction does, read off the draft
//! Confirm signs, and nothing else. Enter never confirms it.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits;
use super::super::page::{edges, lead};
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::rect::Rect;
use crate::wallet::send::{fee_text, Draft};
use crate::wallet::state::State;

const LEAD: &str = "Check it before it goes. Once the network has it, it cannot be taken \
     back, and its fee is spent whether or not it succeeds.";

fn hex(bytes: &[u8]) -> String {
    let mut s = String::from("0x");
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn what(d: &Draft) -> String {
    let nox = super::nox_amount(d.amount);
    match d.purpose {
        Purpose::Approve => format!("Let staking take {nox} NOX"),
        Purpose::Unstake => String::from("Close a position"),
        _ => format!("Stake {nox} NOX"),
    }
}

fn rows(d: &Draft) -> Vec<(&'static str, String)> {
    Vec::from([
        ("Transaction", what(d)),
        ("Contract", crate::wallet::etna::groups::shortened(&hex(&d.to))),
        ("Network", String::from(chain::named(d.chain_id))),
        ("Network fee, at most", fee_text(d.fee_cap_wei())),
        ("Gas limit", format!("{}", d.gas)),
        ("Nonce", format!("{}", d.nonce)),
    ])
}

pub fn review(state: &State, fb: &mut PaintBuffer, d: &Draft) {
    hits::clear();
    let status = super::super::status::parts(state);
    let working = crate::wallet::act::working(state);
    let ready = working.is_none();
    let footer = [
        (working.unwrap_or("Confirm and send"), Weight::Primary, ready),
        ("Back", Weight::Secondary, ready),
    ];
    let spec = FrameSpec {
        number: "07",
        title: "Review stake",
        back: true,
        backdrop: Some(Backdrop::Proving),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    let all = rows(d);
    let at = Rect::new(c.x, y, c.w, row_height() * all.len() as u32);
    tile(fb, at);
    for (i, (name, value)) in all.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == all.len());
    }
    y += at.h;
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ready, ready]);
}
