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

//! A finished fetch's connection, kept for another request or closed.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::{retain, Idle};
use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::net::Source;
use crate::browser::state::State;

/*
 * A response with exact framing leaves its connection for the next request
 * to its host, in the call that read its last byte; anything else is closed
 * then and there. stash() still keeps a TLS image connection in its own
 * slot, and the pool takes it from there at once. The mixnet carries one
 * conversation, which a kept connection would only get in the way of.
 */
/// Keep or close `job`'s connection, now that its response is read.
pub(in crate::browser::fetch) fn free(
    state: &mut State,
    w: &mut NetWire,
    job: &mut Fetch,
    raw: Option<&[u8]>,
) {
    let now = w.now_ms();
    let kept = match raw {
        Some(_) if w.mixnet() => None,
        Some(raw)
            if job.image.is_some()
                && crate::browser::fetch::stash::stash(state, job, Some(raw)) =>
        {
            state.keep.take().map(|k| Idle::kept(k, now))
        }
        Some(raw) => retain(job, raw, now),
        None => None,
    };
    match kept {
        Some(idle) => state.pool.park(w, idle),
        None => w.close(job.handle),
    }
}
