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

//! ACCOUNT, for the wallet: the meter's identity, whether this boot keeps
//! data, its budget and channels (`wire::account_reply`). The body is one
//! byte, the first channel wanted.

use alloc::vec::Vec;

use nonos_nox_meter::wire::account_reply;

use crate::disk::kept;
use crate::meter::{now_ms, Meter};
use crate::protocol::{encode_response, Request, EINVAL};

pub fn account(meter: &mut Meter, req: Request<'_>) -> Vec<u8> {
    let [from] = req.payload else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let who = meter.ready().ok().map(|k| k.identity);
    let now = now_ms() / 1000;
    let body = account_reply(&meter.state, who.as_ref(), kept(), now, *from as usize);
    encode_response(req.seq, 0, &body)
}
