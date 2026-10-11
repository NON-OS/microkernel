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

//! OPEN (op 12), from the wallet, in two calls (plan/21, v1.4.1):
//!
//! - START: the meter checks its gas, encodes approve(settlement, amount)
//!   and open(payee, listingId, amount, expiry, reclaimTo) itself, signs both
//!   with its own key and has settle broadcast them. It answers
//!   approveTx | openTx and keeps nothing.
//! - CONFIRM: by openTx, the meter records the channel only on a
//!   ChannelOpened it asked for (`nonos_nox_meter::open::confirmed`). EAGAIN
//!   while unmined; sent again, it answers the same.

use alloc::vec::Vec;

use nonos_payment_proto::{parse_open, EINVAL};

use super::open_flow::{confirm, start};
use crate::meter::Meter;
use crate::server::Call;

pub fn open(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let run = match parse_open(req.body) {
        Some(Ok(s)) => start(meter, &s),
        Some(Err(c)) => confirm(meter, &c),
        None => Err(EINVAL),
    };
    match run {
        Ok(body) => req.answer(0, &body),
        Err(code) => req.answer(code, &[]),
    }
}
