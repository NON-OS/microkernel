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
//! The drawn draft on the review: the recipient, then every field the
//! confirm signs, one row each.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::chain;
use crate::wallet::etna::parts::tile::{row_height, tile, tile_row};
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::send::exact::exact_text;
use crate::wallet::send::{asset_name, decimals, fee_text, Draft, ASSET_ETH};

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

/// The recipient and the rows of `d`, from `y` down; returns where they end.
pub(super) fn draft(fb: &mut PaintBuffer, c: Rect, mut y: u32, d: &Draft) -> u32 {
    let to = crate::wallet::screen::receive_address::checksummed_text(&d.recipient);
    y += value_block(fb, c.x, y, c.w, &to) + GAP;
    let all = rows(d);
    let at = Rect::new(c.x, y, c.w, row_height() * all.len() as u32);
    tile(fb, at);
    for (i, (name, value)) in all.iter().enumerate() {
        tile_row(fb, at, y + row_height() * i as u32, name, value, i + 1 == all.len());
    }
    y += at.h;
    y
}
