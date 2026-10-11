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

use nonos_nox_meter::caller::Role;
use nonos_payment_proto::{parse_purchase, Purchase, EINVAL, ENOENT};
use nonos_payment_proto::{PURCHASE_POST, PURCHASE_READY, PURCHASE_TAKE, PURCHASE_TAKE_READY};

use crate::jobs::Work;
use crate::meter::now_ms;
use crate::server::Call;

/// Who may send this body: the store posts names and takes the orderId;
/// the wallet takes names and posts the orderId.
pub fn purchase_role(body: &[u8]) -> Option<Role> {
    match *body.first()? {
        PURCHASE_POST | PURCHASE_TAKE_READY => Some(Role::Store),
        PURCHASE_TAKE | PURCHASE_READY => Some(Role::Wallet),
        _ => None,
    }
}

pub fn purchase(work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let now = now_ms();
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
