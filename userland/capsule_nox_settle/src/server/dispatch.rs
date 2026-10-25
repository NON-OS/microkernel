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

use alloc::vec::Vec;

use nonos_settle_proto::{EINVAL, EPERM, OP_BROADCAST, OP_CHANNEL_STATE, OP_FETCH, OP_GAS};
use nonos_settle_proto::{OP_DELIVERED, OP_HEALTHCHECK, OP_INTENT, OP_LISTING, OP_OPENED};
use nonos_settle_proto::{OP_READ, OP_SETTLE};

use super::call::Call;
use super::gate::{admitted, fetch_admitted, flags_admitted};
use crate::handlers;
use crate::settle::Settle;

/// Each op answers only its callers (settle_proto::callers, v1.4.7): every
/// argument reaches an RPC host or a URL, so a capsule without Network must
/// not reach the network through settle. The caller is checked before the
/// body is read. Op 3 (the old READ_NONCE) answers no one.
pub fn dispatch(s: &mut Settle, sender: u32, req: &Call<'_>) -> Vec<u8> {
    if !admitted(req.op, sender) {
        return req.answer(EPERM, &[]);
    }
    if !flags_admitted(req.flags, req.op, sender) {
        return req.answer(EINVAL, &[]);
    }
    if req.op == OP_FETCH && !fetch_admitted(req.body, sender) {
        return req.answer(EPERM, &[]);
    }
    let r = match req.op {
        OP_HEALTHCHECK => Ok(Vec::new()),
        OP_BROADCAST => handlers::broadcast(s, req.body),
        OP_CHANNEL_STATE => handlers::channel_state(s, req.body),
        OP_GAS => handlers::gas(s, req.body),
        OP_OPENED => handlers::opened(s, req.body),
        OP_READ => handlers::read(s, req.body),
        OP_INTENT => handlers::intent(s, req.body),
        OP_FETCH => handlers::fetch(s, sender, req.body),
        OP_SETTLE => handlers::settle_now(s, req.body),
        OP_LISTING => handlers::listing(s, req.body),
        OP_DELIVERED => handlers::delivered(s, req.body),
        _ => Err(EINVAL),
    };
    req.result(r)
}
