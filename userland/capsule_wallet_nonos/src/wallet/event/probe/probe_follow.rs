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

//! Following the payments still on their way, one reading at a time.

use crate::wallet::state::State;

/* Each payment's fate from one reading; the one on the Sent screen also
 * says it there. True when anything shown changed. */
pub(in crate::wallet::event) fn followed_now(
    state: &mut State,
    f: crate::wallet::net::step::Followed,
) -> bool {
    use crate::wallet::send::sent::{Fate, Reading};
    let now = nonos_libc::mk_uptime_ms();
    let mut changed = false;
    for (hash, receipt) in f.receipts {
        let r = Reading { receipt, head: f.head, latest: f.latest, pending: f.pending };
        let Some(fate) = state.broadcast.sent.judge(&hash, r, now) else { continue };
        changed = true;
        if hash != state.broadcast.hash {
            continue;
        }
        match fate {
            Fate::Confirmed { ok } => {
                crate::wallet::send::follow(state, &hash, Some(ok));
            }
            Fate::InBlock { .. } => {
                state.status = b"in a block, waiting for it to be confirmed";
            }
            Fate::Replaced => {
                state.broadcast.receipt_ready = true;
                state.broadcast.receipt_ok = false;
                state.broadcast.unknown = false;
                state.status = b"replaced: another transaction used its nonce";
            }
            Fate::Dropped => {
                state.broadcast.receipt_ready = true;
                state.broadcast.receipt_ok = false;
                state.broadcast.unknown = false;
                state.status = b"dropped: no block has it, nothing was paid";
            }
            Fate::Waiting => {
                state.status = b"sent, waiting for a block";
            }
        }
    }
    changed
}
