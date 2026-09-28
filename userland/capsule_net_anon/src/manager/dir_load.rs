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

//! Taking a verified consensus and starting to join microdescriptors onto it.

use crate::trace;

use super::dir_consensus::obtain as obtain_consensus;
use super::state::{Bootstrap, Manager};

/*
 * A consensus attempt is 376 KB off the wire and 1.8 MB inflated. Retrying one
 * that just failed on the very next turn re-downloads all of it, and a boot that
 * could not reach a quorum did that thirty six times. The wait is longer than the
 * bootstrap's because the thing being waited for is slower: another authority
 * becoming reachable, not a DHCP lease landing.
 */
const RETRY_MS: u64 = 5_000;

pub(super) fn load(state: &mut Manager, now: u64) {
    let Some(doc) = obtain_consensus(state.tcp_port, &state.certs, now) else {
        state.retry_after = now.saturating_add(RETRY_MS);
        return;
    };
    trace::say_num(b"consensus relays", doc.entries.len() as u64);
    state.entries = doc.entries;
    state.weights = doc.weights;
    state.fresh_until = doc.fresh_until;
    state.valid_until = doc.valid_until;
    state.micro.clear();
    state.micro_cursor = 0;
    state.authority_cursor = state.authority_cursor.wrapping_add(1);
    state.bootstrap = Bootstrap::Joining;
}
