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

//! SET_BUDGET, from the wallet: a fresh allowance (`wire::parse_budget`),
//! on disk before it counts.

use alloc::vec::Vec;

use nonos_nox_meter::state::MeterState;
use nonos_nox_meter::wire::parse_budget;

use crate::meter::Meter;
use crate::protocol::{encode_response, Request};
use crate::server::status_of;

pub fn set_budget(meter: &mut Meter, req: Request<'_>) -> Vec<u8> {
    let budget = match parse_budget(req.payload) {
        Ok(b) => b,
        Err(e) => return encode_response(req.seq, status_of(e), &[]),
    };
    // The channels come from the file, so it must have been read first.
    if let Err(code) = meter.ready() {
        return encode_response(req.seq, code, &[]);
    }
    let next = MeterState { budget, channels: meter.state.channels.clone() };
    match meter.commit(next) {
        Ok(()) => encode_response(req.seq, 0, &[]),
        Err(code) => encode_response(req.seq, code, &[]),
    }
}
