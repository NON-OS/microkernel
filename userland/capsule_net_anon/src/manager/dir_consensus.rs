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

//! Fetching a consensus and proving the authorities signed it, one authority
//! at a time.

use crate::directory::authority::AUTHORITIES;
use crate::directory::consensus::{parse, Consensus};
use crate::directory::fetch::CONSENSUS_PATH;
use crate::directory::verify::AuthorityCert;
use crate::trace;

use super::dir_job::{turn, Turn};
use super::state::Manager;

/// Advance the sweep by one turn. `None` while it runs; `Some(None)` once
/// every authority was asked and none served a consensus that verified.
pub(super) fn sweep(state: &mut Manager, now: u64) -> Option<Option<Consensus>> {
    let authority = &AUTHORITIES[state.dir.sweep];
    let doc = match turn(state, authority.address, authority.dir_port, CONSENSUS_PATH) {
        Turn::Busy => return None,
        Turn::Got(body) => verified(&body, &state.certs, now),
        Turn::Missed => None,
    };
    state.dir.sweep += 1;
    if doc.is_some() || state.dir.sweep >= AUTHORITIES.len() {
        state.dir.sweep = 0;
        return Some(doc);
    }
    None
}

fn verified(body: &[u8], certs: &[(usize, AuthorityCert)], now: u64) -> Option<Consensus> {
    let Some(doc) = parse(body) else {
        /* With the length, so a consensus that inflated to a few hundred bytes
         * is told apart from one that inflated to a megabyte. */
        trace::say_num(b"consensus parse failed, inflated bytes", body.len() as u64);
        return None;
    };
    /*
     * Signatures before the window, deliberately. Checked the other way round, a
     * document rejected for its dates could be a forgery or could be this
     * machine's clock, and those need different answers.
     */
    if !super::dir_quorum::signed(&doc, body, certs) {
        return None;
    }
    if !doc.valid_at(now) {
        trace::say(b"signed consensus outside its window, check the clock");
        trace::say_two(b"now against valid-after", now, doc.valid_after);
        return None;
    }
    Some(doc)
}
