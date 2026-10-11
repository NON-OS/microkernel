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

//! A frame from settle in the service inbox: the answer to the running
//! job's step, believed only from settle's registry pid, with settle_proto's
//! magic and that step's op and request id (v1.4.15). Anything else from
//! settle on its wire, a late answer included, is dropped unanswered.

use nonos_libc::mk_service_lookup;
use nonos_payment_proto::EIO;
use nonos_settle_proto::{async_answer, parse_header, Gas, Page, ReplyError, OP_FETCH, OP_GAS};
use nonos_settle_proto::{OP_READ, SERVICE};

use super::ask::{hash, words};
use super::drive::advance;
use super::work::Work;
use crate::keccak::Host;
use crate::keys::Dalek;
use crate::meter::{now_ms, Meter};
use crate::server::status_of;

fn settle_pid() -> u32 {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc == 0 {
        pid
    } else {
        0
    }
}

/// Whether `rx` was settle's, and so taken here rather than dispatched.
pub(crate) fn on_frame(work: &mut Work, meter: &Meter, sender: u32, rx: &[u8]) -> bool {
    if parse_header(rx).is_none() || sender == 0 || sender != settle_pid() {
        return false;
    }
    let Some(run) = work.run.as_mut() else { return true };
    let Some(p) = run.pending.as_ref() else { return true };
    let op = p.op;
    let got = async_answer(rx, sender, p.settle_pid, op, p.req);
    if matches!(got, Err(ReplyError::Stale | ReplyError::Foreign)) {
        return true;
    }
    run.pending = None;
    let now = now_ms();
    let next = match got {
        Err(ReplyError::Status(s)) => Err(s),
        Err(_) => Err(EIO),
        Ok(body) => match op {
            OP_READ => words(body).ok_or(EIO).and_then(|w| run.flow.on_view(w).map_err(status_of)),
            OP_GAS => Gas::parse(body).ok_or(EIO).and_then(|g| {
                run.flow.on_gas(&g, &meter.state, (&run.contracts, now / 1000)).map_err(status_of)
            }),
            OP_FETCH => Page::parse(body)
                .ok_or(EIO)
                .and_then(|p| run.flow.on_page(&p, (&Host, &Dalek)).map_err(status_of)),
            _ => hash(body).map(|h| run.flow.on_hash(&h)).ok_or(EIO),
        },
    };
    advance(work, next, now);
    true
}
