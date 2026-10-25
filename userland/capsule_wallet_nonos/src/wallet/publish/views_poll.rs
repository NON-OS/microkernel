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

//! The publisher screen's views (views.rs) through settle's READ while no
//! transaction is under way: the next one asked when due, on the network
//! now, and its answer taken, or asked again after EVERY_MS.

use alloc::vec::Vec;

use nonos_settle_proto::parse_read_reply;

use super::kec::Kec;
use super::send_drive::EVERY_MS;
use super::sending::Asked;
use super::views::Views;
use crate::wallet::chain;
use crate::wallet::state::State;

pub(super) fn ask(state: &mut State, now_ms: u64) -> Option<(Asked, Vec<u8>)> {
    let p = &mut state.publisher;
    let e = p.entry.as_ref()?;
    let chain_id = chain::current().id;
    if p.views.chain_id != chain_id {
        p.views = Views { chain_id, ..Views::default() };
    }
    let r = (now_ms >= p.views.next_ms).then(|| p.views.next(&Kec, e, &state.acct.address))??;
    Some((Asked::View(r.view), r.encode()?))
}

/// Whether the screen changed.
pub(super) fn took(state: &mut State, view: u16, r: Result<Vec<u8>, i32>, now_ms: u64) -> bool {
    let views = &mut state.publisher.views;
    let words = r.ok().and_then(|b| parse_read_reply(&b));
    if !words.is_some_and(|w| views.took(view, w)) {
        views.next_ms = now_ms + EVERY_MS;
    }
    true
}
