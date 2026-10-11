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

//! SET_BUDGET, from the wallet: a fresh allowance (payment_proto::SetBudget),
//! on disk before it counts. On a boot that keeps nothing it is refused, so
//! the budget stays zero.

use alloc::vec::Vec;

use nonos_nox_meter::state::MeterState;
use nonos_nox_meter::wire::budget as allowance;
use nonos_payment_proto::{SetBudget, EINVAL};

use crate::meter::Meter;
use crate::server::{status_of, Call};

pub fn set_budget(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let Some(b) = SetBudget::parse(req.body) else {
        return req.answer(EINVAL, &[]);
    };
    let budget = match allowance(&b) {
        Ok(b) => b,
        Err(e) => return req.answer(status_of(e), &[]),
    };
    // The channels come from the file, so it must have been read first.
    if let Err(code) = meter.ready() {
        return req.answer(code, &[]);
    }
    let next = MeterState { budget, ..meter.state.clone() };
    match meter.commit(next) {
        Ok(()) => req.answer(0, &[]),
        Err(code) => req.answer(code, &[]),
    }
}
