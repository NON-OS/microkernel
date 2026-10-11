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

use nonos_settle_proto::{EINVAL, OP_BROADCAST, OP_CHANNEL_STATE, OP_FETCH, OP_GAS};
use nonos_settle_proto::{OP_HEALTHCHECK, OP_INTENT, OP_OPENED, OP_READ};

use super::call::Call;
use crate::handlers;
use crate::settle::Settle;

/// Every op is open to any caller: settle holds no key, and what it answers
/// is the chain's, or a broadcast of bytes another signed. Op 3 (the old
/// READ_NONCE) is READ's view 14 now, and EINVAL like any unknown op.
pub fn dispatch(s: &mut Settle, req: &Call<'_>) -> Vec<u8> {
    let r = match req.op {
        OP_HEALTHCHECK => Ok(Vec::new()),
        OP_BROADCAST => handlers::broadcast(s, req.body),
        OP_CHANNEL_STATE => handlers::channel_state(s, req.body),
        OP_GAS => handlers::gas(s, req.body),
        OP_OPENED => handlers::opened(s, req.body),
        OP_READ => handlers::read(s, req.body),
        OP_INTENT => handlers::intent(s, req.body),
        OP_FETCH => handlers::fetch(s, req.body),
        _ => Err(EINVAL),
    };
    req.result(r)
}
