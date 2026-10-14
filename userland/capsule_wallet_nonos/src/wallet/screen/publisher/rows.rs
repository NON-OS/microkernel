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

//! The entry's binding, each release's attest, and the rows that sign them.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_marketplace_abi::MarketplaceEntry;

use crate::wallet::chain;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::publish::{why, Kec};
use crate::wallet::publisher::{attest_of, attest_rows, binding_plan};
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::State;

pub const ROW_CONFIRM: u8 = 0;
pub const ROW_BINDING: u8 = 1;
/// Release i's attest is row ROW_ATTEST + i.
pub const ROW_ATTEST: u8 = 2;

pub fn entry(state: &State, e: &MarketplaceEntry, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let mut h = 0;
    match binding_plan(&Kec, e, &state.acct.address) {
        Ok(plan) => plan.rows.iter().for_each(|(k, v)| h += fact(fb, c.x, y + h, c.w, k, v)),
        Err(r) => h += fact(fb, c.x, y + h, c.w, "binding", why(r)),
    }
    let confirmed = state.publisher.confirmed;
    let confirm =
        if confirmed { "Publisher key confirmed" } else { "Confirm: this publisher key is mine" };
    let mut actions: Vec<(String, bool)> =
        Vec::from([(confirm.into(), true), ("Sign the binding".into(), confirmed)]);
    for (i, rel) in e.releases.iter().take(16).enumerate() {
        match attest_of(&Kec, e, i, chain::current().id) {
            Ok(a) => {
                attest_rows(e, i, &a).iter().for_each(|(k, v)| h += fact(fb, c.x, y + h, c.w, k, v))
            }
            Err(r) => h += fact(fb, c.x, y + h, c.w, "release", why(r)),
        }
        actions.push((format!("Sign the attest of {}", rel.release_id), true));
    }
    let rows: Vec<(&str, bool)> = actions.iter().map(|(t, on)| (t.as_str(), *on)).collect();
    let mut at = alloc::vec![Rect::new(0, 0, 0, 0); rows.len()];
    h += GAP + quiet_group(fb, c.x, y + h + GAP, c.w, &rows, &mut at);
    for (i, r) in at.iter().enumerate().filter(|(i, _)| rows[*i].1) {
        hits::put(Press::Row(i as u8), *r);
    }
    h
}
