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

//! PAY v2: a charge for exactly what a release sells, as a signed
//! cumulative voucher. In order:
//!
//! 1. the body parses (payment_proto), and the meter's key and state are
//!    ready; on a boot that keeps nothing there is no key (ENOPERSIST);
//! 2. the market says the release's commerce block is trusted, and every
//!    word of the request equals it (`nonos_nox_meter::pay::priced`);
//! 3. settle refreshes the channel if its copy is stale, best effort;
//! 4. the channel and the budget take the charge, and the new state reaches
//!    disk;
//! 5. only then does the keyring sign the voucher.
//!
//! The reply is the signed voucher, one payment_proto::VoucherRecord.

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::pay::priced;
use nonos_nox_meter::wire::record;
use nonos_payment_proto::{PayRequest, EINVAL, ENOENT, VOUCHER_RECORD_LEN};

use super::pay_charge::charge;
use crate::clients::{offer, sign_voucher};
use crate::meter::Meter;
use crate::server::{status_of, Call};

pub fn pay(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    match run(meter, req.body) {
        Ok(record) => req.answer(0, &record),
        Err(code) => req.answer(code, &[]),
    }
}

fn run(meter: &mut Meter, body: &[u8]) -> Result<Vec<u8>, i32> {
    let r = PayRequest::parse(body).ok_or(EINVAL)?;
    let keys = meter.ready()?;
    let nox = address(r.chain_id, Contract::NoxToken).map_err(|_| ENOENT)?;
    let o = offer(r.listing, r.release)?;
    let amount = priced(&r, &o, &nox).map_err(status_of)?;
    meter.refresh_stale(&r.payee, &r.listing_id, r.chain_id);
    let c = charge(meter, &r, amount)?;
    let v = c.voucher(&keys.address);
    let sig = sign_voucher(keys.key_id, r.chain_id, &v)?;
    meter.signed(&c.id, c.seq, sig);
    let mut out = Vec::with_capacity(VOUCHER_RECORD_LEN);
    record(&v, &sig).put(&mut out);
    Ok(out)
}
