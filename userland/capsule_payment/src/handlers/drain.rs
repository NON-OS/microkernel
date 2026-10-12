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

//! DRAIN (op 3), for capsule_nox_settle only, in two forms
//! (payment_proto::DrainRequest):
//!
//! - LIST: the newest signed voucher of each channel, a page from `from`.
//! - SIGN: settle(vouchers, sigs) from the meter's own newest vouchers of
//!   the named channels, signed with its own key. The state recording what
//!   it carries reaches disk first, so a crash can lose a settle but never
//!   let the meter sign one more than the bound allows (`settle_plan`).

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::settle::settle_plan;
use nonos_nox_meter::wire::drain as view;
use nonos_payment_proto::{DrainRequest, DrainSign, EINVAL, ENOENT, EOVERFLOW};

use crate::keyring::sign_tx;
use crate::meter::{now_ms, Meter};
use crate::server::{status_of, Call};

pub fn drain(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    let run = match DrainRequest::parse(req.body) {
        Some(DrainRequest::List { from }) => list(meter, from),
        Some(DrainRequest::Sign(s)) => sign(meter, &s),
        None => Err(EINVAL),
    };
    req.result(run)
}

fn list(meter: &mut Meter, from: u8) -> Result<Vec<u8>, i32> {
    let k = meter.ready()?;
    view(&meter.state, &k.address, from as usize).encode().ok_or(EOVERFLOW)
}

fn sign(meter: &mut Meter, s: &DrainSign) -> Result<Vec<u8>, i32> {
    let k = meter.ready()?;
    let settlement = address(s.chain_id, Contract::Settlement).map_err(|_| ENOENT)?;
    let now = now_ms() / 1000;
    let (next, tx) =
        settle_plan(&meter.state, &k.address, s, &settlement, now).map_err(status_of)?;
    meter.commit(next)?;
    sign_tx(k.key_id, &tx)
}
