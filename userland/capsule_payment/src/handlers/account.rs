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

//! ACCOUNT, for the wallet: the meter's address, whether this boot keeps
//! data, its budget and its channels (payment_proto::Account). The body is
//! one byte, the first channel wanted. It answers on every boot, a boot
//! that keeps nothing included: that is how the wallet learns why.

use alloc::vec::Vec;

use nonos_nox_meter::wire::account as view;
use nonos_payment_proto::{EINVAL, EOVERFLOW};

use crate::disk::kept;
use crate::meter::{now_ms, Meter};
use crate::server::Call;

pub fn account(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let [from] = req.body else {
        return req.answer(EINVAL, &[]);
    };
    let address = meter.ready().ok().map(|k| k.address);
    let a = view(&meter.state, address, kept(), now_ms() / 1000, *from as usize);
    match a.encode() {
        Some(body) => req.answer(0, &body),
        None => req.answer(EOVERFLOW, &[]),
    }
}
