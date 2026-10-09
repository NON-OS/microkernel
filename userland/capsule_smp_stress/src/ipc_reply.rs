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

//! The asking side's wait for its answer, in an IPC round trip.

use crate::clock::now_ms;
use crate::ipc_pair::OWN_INBOX;
use crate::stall;
use crate::stats::{self, Kind};
use nonos_libc::mk_ipc_recv;

/// What `mk_ipc_recv` answers when its wait ran out.
const TIMED_OUT: i64 = -110;

/// Wait for `seq` to come back. A reply to an earlier round that came late
/// is passed over.
pub fn await_reply(seq: u64, buf: &mut [u8; 8]) {
    let sent_at = now_ms();
    while !stats::stopping() {
        let got = mk_ipc_recv(OWN_INBOX, buf.as_mut_ptr(), buf.len(), stall::WAIT_TIMEOUT_MS);
        let waited = now_ms().saturating_sub(sent_at);
        if got == 8 && u64::from_le_bytes(*buf) == seq {
            stats::round(Kind::Ipc, waited);
            if stall::is_stall(waited) {
                stats::fault(Kind::Stall);
            }
            return;
        }
        if got == TIMED_OUT && !stats::stopping() {
            // Nothing came back within the wait: the reply's wake was lost.
            stats::fault(Kind::Stall);
            return;
        }
        if got < 0 && got != TIMED_OUT {
            stats::fault(Kind::Error);
            return;
        }
    }
}
