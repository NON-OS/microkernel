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

//! Fetching the microdescriptors a consensus points at.

use crate::crypto::sha256;
use crate::directory::microdesc::{parse, pieces};
use crate::trace;

use super::batch::{batch_at, batch_count, batch_entries};
use super::http::fetch;
use super::state::Manager;

pub(super) struct Batch {
    pub added: usize,
    pub last: bool,
}

/*
 * The accumulator is kept ordered by digest, and both the duplicate check and
 * the join search it rather than scan it. Five thousand against five thousand
 * is twenty five million comparisons, once per turn for the whole bootstrap.
 */

pub(super) fn gather_one(state: &mut Manager) -> Batch {
    let total = batch_count(&state.entries);
    let Some((address, dir_port, path)) =
        batch_at(&state.entries, state.authority_cursor, state.micro_cursor)
    else {
        return Batch { added: 0, last: true };
    };
    let last = state.micro_cursor + 1 >= total;
    let asked = state.micro_cursor;
    state.micro_cursor += 1;
    let Some(body) = fetch(state.tcp_port, address, dir_port, &path) else {
        return Batch { added: 0, last };
    };
    let mut added = 0usize;
    for (from, to) in pieces(&body) {
        let piece = &body[from..to];
        let Ok(digest) = sha256(piece) else { continue };
        // Only the digests this request asked for. Accepting one from any batch is a
        // looser rule than the one wanted: an answer should contain what was asked.
        if !batch_entries(&state.entries, asked).iter().any(|e| e.microdesc_digest == digest) {
            continue;
        }
        let Err(at) = state.micro.binary_search_by(|(held, _)| held.cmp(&digest)) else {
            continue;
        };
        if let Some(micro) = parse(piece) {
            state.micro.insert(at, (digest, micro));
            added += 1;
        }
    }
    trace::say_two(b"microdescs", added as u64, state.micro.len() as u64);
    Batch { added, last }
}
