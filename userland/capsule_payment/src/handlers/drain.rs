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

//! DRAIN, for capsule_nox_settle: the newest signed voucher of each channel
//! (`wire::drain_reply`). The body is one byte, the first voucher wanted.

use alloc::vec::Vec;

use nonos_nox_meter::wire::drain_reply;

use crate::meter::Meter;
use crate::protocol::{encode_response, Request, EINVAL};

pub fn drain(meter: &mut Meter, req: Request<'_>) -> Vec<u8> {
    let [from] = req.payload else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    match meter.ready() {
        Ok(k) => encode_response(
            req.seq,
            0,
            &drain_reply(&meter.state, &k.identity.address, *from as usize),
        ),
        Err(code) => encode_response(req.seq, code, &[]),
    }
}
