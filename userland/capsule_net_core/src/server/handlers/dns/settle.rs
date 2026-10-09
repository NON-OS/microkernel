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

//! Answers the callers of waiting lookups once the serve loop has polled.

use nonos_libc::mk_uptime_ms;

use super::answer::{cancel, err, read};
use super::lookups::PENDING;
use super::pending::Look;
use super::unanswered::say_unanswered;
use super::verdict::{failure_errno, Found};
use crate::protocol::dns::{E_OK, E_SERVFAIL, E_TIMEOUT, MAGIC_NDNS, OP_RESOLVE_A};
use crate::server::respond::reply;
use crate::state;

/// Answer every waiting lookup whose answer is in, failed on every server,
/// or out of time.
pub fn settle(tx: &mut [u8]) {
    let now = mk_uptime_ms();
    PENDING.lock().sweep(now, |w, expired| {
        let (generation, dns) = (w.query.generation, w.query.dns);
        let found = state::with_dns_at(generation, dns, |sockets| read(sockets, &mut w.query));
        if found == Some(Found::Pending) && !expired {
            return Look::Pending;
        }
        // Whatever ends the lookup, a query still unread on another server is
        // freed, so its slot does not hold an answer no one will read.
        cancel(&mut w.query);
        match found {
            Some(Found::Address(ip)) => {
                let _ = reply(w.pid, MAGIC_NDNS, OP_RESOLVE_A, E_OK, w.request_id, &ip, tx);
            }
            // A socket set replaced since the lookup started counts as a
            // server that failed it.
            other => {
                let errno = failure_errno(other.unwrap_or(Found::Failed));
                if errno == E_TIMEOUT {
                    say_unanswered(now);
                }
                err(w.pid, w.request_id, errno, tx)
            }
        }
        Look::Answered
    });
}

/// Answer the lookup `pid` gave up waiting on before anything answers its new
/// call: the kernel hands a caller's replies to its calls in order.
pub fn settle_for(pid: u32, tx: &mut [u8]) {
    let Some(mut w) = PENDING.lock().take_for(pid) else { return };
    cancel(&mut w.query);
    err(w.pid, w.request_id, E_SERVFAIL, tx);
}
