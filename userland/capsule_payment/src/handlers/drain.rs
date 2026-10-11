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
//! (payment_proto::Drain). The body is one byte, the first voucher wanted.

use alloc::vec::Vec;

use nonos_nox_meter::wire::drain as view;
use nonos_payment_proto::EINVAL;

use crate::meter::Meter;
use crate::server::Call;

pub fn drain(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let [from] = req.body else {
        return req.answer(EINVAL, &[]);
    };
    match meter.ready() {
        Ok(k) => req.answer(0, &view(&meter.state, &k.address, *from as usize).encode()),
        Err(code) => req.answer(code, &[]),
    }
}
