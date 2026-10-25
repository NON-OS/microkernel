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

//! PURCHASE (op 19, the plan, the plan, v1.4.42): the store-to-wallet mailbox.
//! 19/0 POST from app.store, names only, EBUSY while one waits; 19/1 TAKE
//! from the wallet, the names once, ENOENT when none wait. 19/2 READY from
//! the wallet, the names and their orderId; 19/3 TAKE_READY from app.store.

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use nonos_payment_proto::{parse_purchase, Purchase, EINVAL, ENOENT};

use crate::jobs::Work;
use crate::server::Call;

pub fn purchase(work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    // The hold is a timeout, so it runs on the monotonic clock: a step of
    // the wall clock neither keeps a posting past HOLD_MS nor drops it early.
    let now = mk_uptime_ms().max(0) as u64;
    let m = &mut work.mailbox;
    req.result(match parse_purchase(req.body) {
        Some(Purchase::Post(names)) => m.names.post(names, now).map(|_| Vec::new()),
        Some(Purchase::Take) => {
            m.names.take(now).ok_or(ENOENT).and_then(|n| n.encode().ok_or(EINVAL))
        }
        Some(Purchase::Ready(r)) => m.ready.post(r, now).map(|_| Vec::new()),
        Some(Purchase::TakeReady) => {
            m.ready.take(now).ok_or(ENOENT).and_then(|r| r.encode().ok_or(EINVAL))
        }
        None => Err(EINVAL),
    })
}
