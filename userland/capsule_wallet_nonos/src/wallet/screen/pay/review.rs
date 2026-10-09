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
//! The review: every field the confirm signs, and the one sentence on what
//! becomes public. Confirm is the only thing that signs.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits;
use super::super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::send::exact::exact_text;
use crate::wallet::send::{asset_name, decimals, fee_text, Draft, ASSET_ETH};
use crate::wallet::state::State;

const LEAD: &str = "Check it before it goes. A payment cannot be taken back once the \
     network has it.";

fn rows(d: &Draft) -> Vec<(&'static str, String)> {
    let amount = format!("{} {}", exact_text(d.amount, decimals(d.asset)), asset_name(d.asset));
    let mut out = Vec::from([
        ("Send", amount),
        ("Network", String::from(chain::named(d.chain_id))),
        ("Network fee, at most", fee_text(d.fee_cap_wei())),
        ("Gas limit", format!("{}", d.gas)),
        ("Nonce", format!("{}", d.nonce)),
    ]);
    if d.asset == ASSET_ETH {
        out.push(("Total, at most", fee_text(d.amount.saturating_add(d.fee_cap_wei()))));
    } else {
        let contract = crate::wallet::screen::receive_address::checksummed_text(&d.to);
        out.push(("Token contract", crate::wallet::etna::groups::shortened(&contract)));
    }
    out
}

pub fn review(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    /* While the payment goes out the buttons say so and take no press. */
    let working = crate::wallet::act::working(state);
    let ready = state.send_draft.is_some() && working.is_none();
    let footer = [
        (working.unwrap_or("Confirm and send"), Weight::Primary, ready),
        ("Edit", Weight::Secondary, working.is_none()),
    ];
    let spec = FrameSpec {
        number: "05",
        title: "Review",
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
    if let Some(d) = &state.send_draft {
        let to = crate::wallet::screen::receive_address::checksummed_text(&d.recipient);
        y += value_block(fb, c.x, y, c.w, &to) + GAP;
        let all = rows(d);
        let at = Rect::new(c.x, y, c.w, row_height() * all.len() as u32);
        tile(fb, at);
        for (i, (name, value)) in all.iter().enumerate() {
            tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == all.len());
        }
        y += at.h;
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ready, working.is_none()]);
}
