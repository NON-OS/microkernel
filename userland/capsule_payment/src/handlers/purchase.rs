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

//! PURCHASE (op 19, plan/21, plan/27): the store-to-wallet mailbox. 19/0
//! POST from app.store alone, names only, EBUSY while one waits; 19/1 TAKE
//! from the wallet alone, the names once, ENOENT when none wait.

use alloc::vec::Vec;

use nonos_nox_meter::caller::Role;
use nonos_payment_proto::{parse_purchase, EINVAL, ENOENT, PURCHASE_POST, PURCHASE_TAKE};

use crate::jobs::Work;
use crate::meter::now_ms;
use crate::server::Call;

/// Who may send this body: the store posts, the wallet takes.
pub fn purchase_role(body: &[u8]) -> Option<Role> {
    match *body.first()? {
        PURCHASE_POST => Some(Role::Store),
        PURCHASE_TAKE => Some(Role::Wallet),
        _ => None,
    }
}

pub fn purchase(work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let now = now_ms();
    req.result(match parse_purchase(req.body) {
        Some(Some(names)) => work.mailbox.post(names, now).map(|_| Vec::new()),
        Some(None) => work.mailbox.take(now).ok_or(ENOENT).and_then(|n| n.encode().ok_or(EINVAL)),
        None => Err(EINVAL),
    })
}
