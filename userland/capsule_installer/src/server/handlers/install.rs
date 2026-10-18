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

//! OP_INSTALL: pay the meter for one install of a listing, as PAY v2
//!. The body is a payment_proto PayRequest for exactly one unit
//! of "install"; the installer checks that much and relays it, and the meter
//! checks the rest against the release's signed commerce block. The reply is
//! the meter's signed voucher. A free listing needs no call at all.
//!
//! The voucher reaches the chain before the install goes on (v1.4.7): the
//! installer asks settle for a drain round now, and answers the voucher
//! only when the settle() it broadcast carries it. Nothing broadcast, or a
//! settle() without it, fails the install.

use alloc::vec::Vec;

use nonos_nox_config::UNIT_INSTALL;
use nonos_payment_proto::{PayRequest, VoucherRecord, EIO};

use super::super::pay_call::request_payment;
use super::super::settle_call::settle_now;
use crate::protocol::{encode_response, Request, EINVAL};

pub fn install(req: Request<'_>) -> Vec<u8> {
    let Some(pay) = PayRequest::parse(req.payload) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if pay.unit != UNIT_INSTALL || pay.qty != 1 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    match request_payment(req.payload).and_then(|v| settled(pay.chain_id, v)) {
        Ok(voucher) => encode_response(req.seq, 0, &voucher),
        Err(e) => encode_response(req.seq, e, &[]),
    }
}

/// `voucher`, once a settle() carrying it has been broadcast.
fn settled(chain_id: u64, voucher: Vec<u8>) -> Result<Vec<u8>, i32> {
    let v = VoucherRecord::parse(&voucher).ok_or(EIO)?;
    let round = settle_now(chain_id)?;
    round.carries(&v.channel_id, v.cumulative).then_some(voucher).ok_or(EIO)
}
