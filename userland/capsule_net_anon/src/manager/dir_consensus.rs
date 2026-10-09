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

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::authority::AUTHORITIES;
use crate::directory::consensus::{parse, Consensus};
use crate::directory::fetch::CONSENSUS_PATH;
use crate::directory::verify::AuthorityCert;
use crate::trace;

use super::dir_job::{turn, Turn};
use super::state::Manager;

/// What one authority's consensus came to.
pub(super) enum Fetched {
    /// Signed by a quorum and inside its window.
    Doc(Consensus),
    /// Short of its quorum, and naming signing keys the client holds no
    /// certificate for: those authorities, to be asked again.
    Unheld(Vec<usize>),
    /// Nothing usable from any authority this sweep.
    Nothing,
}

/// Advance the sweep by one turn. `None` while it runs; `Some` once an
/// authority's consensus verified, or named keys not held, or every
/// authority was asked and none gave either.
pub(super) fn sweep(state: &mut Manager, now: u64) -> Option<Fetched> {
    let authority = &AUTHORITIES[state.dir.sweep];
    let fetched = match turn(state, authority.address, authority.dir_port, CONSENSUS_PATH) {
        Turn::Busy => return None,
        Turn::Got(body) => verified(&body, &state.certs, now),
        Turn::Missed => Fetched::Nothing,
    };
    state.dir.sweep += 1;
    if !matches!(fetched, Fetched::Nothing) || state.dir.sweep >= AUTHORITIES.len() {
        state.dir.sweep = 0;
        return Some(fetched);
    }
    None
}

fn verified(body: &[u8], certs: &[(usize, AuthorityCert)], now: u64) -> Fetched {
    let Some(doc) = parse(body) else {
        /* With the length, so a consensus that inflated to a few hundred bytes
         * is told apart from one that inflated to a megabyte. */
        trace::say_num(b"consensus parse failed, inflated bytes", body.len() as u64);
        return Fetched::Nothing;
    };
    /*
     * Signatures before the window, deliberately. Checked the other way round, a
     * document rejected for its dates could be a forgery or could be this
     * machine's clock, and those need different answers.
     */
    let Some(count) = super::dir_quorum::signed(&doc, body, certs) else {
        let missing = super::dir_quorum::unheld_in(&doc, certs);
        return if missing.is_empty() { Fetched::Nothing } else { Fetched::Unheld(missing) };
    };
    let mut doc = doc;
    doc.verified_signatures = u8::try_from(count).unwrap_or(u8::MAX);
    if !doc.valid_at(now) {
        trace::say(b"signed consensus outside its window, check the clock");
        trace::say_two(b"now against valid-after", now, doc.valid_after);
        return Fetched::Nothing;
    }
    Fetched::Doc(doc)
}
