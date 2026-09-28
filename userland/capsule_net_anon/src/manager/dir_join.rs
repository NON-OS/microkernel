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

//! Bringing microdescriptors in one request at a time.

use crate::path::draw_path;
use crate::trace;

use super::dir_micro::gather_one;
use super::relays::join;
use super::state::{Bootstrap, Manager};

/*
 * One request per turn, and the relay set rebuilt after each. Waiting for all
 * fifty five before answering anything cost minutes of a frozen serve loop to
 * gain relays a first circuit does not need: a path wants one guard, one middle
 * and one exit that survive the weights and the network rule, and a few hundred
 * relays already offer that. The rest keep arriving behind a working transport
 * and widen the pool as they land.
 */

/// Fetch the next batch and promote to Ready once a path can be drawn.
pub fn tick(state: &mut Manager) {
    let batch = gather_one(state);
    if batch.added == 0 && !batch.last {
        return;
    }
    state.relays = join(&state.entries, &state.micro);
    if draw_path(&state.relays, &state.weights).is_some() {
        trace::say_two(b"directory usable", state.relays.len() as u64, state.entries.len() as u64);
        state.bootstrap = Bootstrap::Ready;
        return;
    }
    if batch.last {
        /*
         * Every batch asked for and still no path. Not a stall to sit in: the
         * consensus may have been served with microdescriptors the authorities no
         * longer hold, so the whole document is dropped and fetched again rather
         * than retried against the same digests for ever.
         */
        trace::say(b"every microdescriptor asked for and no path, refetching");
        state.bootstrap = Bootstrap::Anchored;
    }
}
