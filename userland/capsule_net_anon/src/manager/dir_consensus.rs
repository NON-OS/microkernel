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

//! Fetching a consensus and proving the authorities signed it.

extern crate alloc;

use crate::directory::authority::AUTHORITIES;
use crate::directory::consensus::{parse, Consensus};
use crate::directory::fetch::CONSENSUS_PATH;
use crate::directory::verify::AuthorityCert;
use crate::trace;

use super::http::fetch;

/// A consensus that parsed, verified and is valid at `now`, with its bytes.
///
pub fn obtain(tcp_port: u32, certs: &[(usize, AuthorityCert)], now: u64) -> Option<Consensus> {
    for authority in AUTHORITIES.iter() {
        let Some(body) = fetch(tcp_port, authority.address, authority.dir_port, CONSENSUS_PATH)
        else {
            continue;
        };
        let Some(doc) = parse(&body) else {
            // With the length, so a consensus that inflated to a few hundred bytes is told
            // apart from one that inflated to a megabyte.
            trace::say_num(b"consensus parse failed, inflated bytes", body.len() as u64);
            continue;
        };
        /*
         * Signatures before the window, deliberately. Checked the other way round, a
         * document rejected for its dates could be a forgery or could be this machine's
         * clock, and those need different answers.
         */
        if !super::dir_quorum::signed(&doc, &body, certs) {
            continue;
        }
        if !doc.valid_at(now) {
            trace::say(b"signed consensus outside its window, check the clock");
            trace::say_two(b"now against valid-after", now, doc.valid_after);
            continue;
        }
        return Some(doc);
    }
    None
}
