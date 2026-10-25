// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The block wire's legs of the leave phase (block_device::leave): one per
//! endpoint whose capsule runs, a flush of each unit it may serve, in turn,
//! inside one budget, under the endpoint's own call lock.

use alloc::vec::Vec;

use super::codec::{decode_response, encode_request};
use super::header::OP_FLUSH;
use super::table::{count, endpoint};
use crate::hardware::block_device::leave::{Leg, Wire};
use crate::services::lifecycle::transport::next_request_id;

const BUDGET_MS: u64 = 5_000;

pub(in crate::hardware::block_device) fn leave_legs() -> Vec<Leg> {
    (0..count())
        .filter_map(endpoint)
        .filter_map(|e| {
            let wire = Wire {
                lock: &e.slot.call,
                sender: e.sender,
                reply_inbox: e.reply_inbox,
                state: e.state,
                decode: decode_response,
            };
            let units = (0..e.units.min(super::endpoint::MAX_UNITS as u16)).map(|unit| {
                let id = next_request_id(&e.slot.seq);
                (id, encode_request(OP_FLUSH, unit, id, &[]))
            });
            Leg::new(e.shown, BUDGET_MS, wire, units.collect())
        })
        .collect()
}
