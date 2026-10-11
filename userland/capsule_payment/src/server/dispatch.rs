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

use nonos_nox_meter::caller::Role;

use super::gate::admits;
use crate::handlers;
use crate::meter::Meter;
use crate::protocol::{
    encode_response, Request, EINVAL, EPERM, OP_ACCOUNT, OP_DRAIN, OP_HEALTHCHECK, OP_LIST_TOKENS,
    OP_PAY, OP_SET_BUDGET,
};

/// Each op with the role its caller must play. The caller is
/// checked before the body is read.
pub fn dispatch(meter: &mut Meter, sender: u32, req: Request<'_>) -> Vec<u8> {
    let role = match req.op {
        OP_HEALTHCHECK | OP_LIST_TOKENS => Role::Anyone,
        OP_PAY => Role::Payer,
        OP_DRAIN => Role::Settle,
        OP_ACCOUNT | OP_SET_BUDGET => Role::Wallet,
        _ => return encode_response(req.seq, EINVAL, &[]),
    };
    if !admits(role, sender) {
        return encode_response(req.seq, EPERM, &[]);
    }
    match req.op {
        OP_HEALTHCHECK => encode_response(req.seq, 0, &[]),
        OP_LIST_TOKENS => handlers::tokens(req),
        OP_PAY => handlers::pay(meter, req),
        OP_DRAIN => handlers::drain(meter, req),
        OP_ACCOUNT => handlers::account(meter, req),
        _ => handlers::set_budget(meter, req),
    }
}
