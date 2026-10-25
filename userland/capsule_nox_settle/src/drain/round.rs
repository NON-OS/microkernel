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

//! One drain round on a chain: the meter's vouchers,
//! what is owed, the meter's gas, an estimate, the meter's own signature,
//! then the broadcast. Settle encodes settle() here only to estimate its
//! gas; the meter rebuilds it from its own vouchers, so this encoding never
//! reaches a signature.

use alloc::vec::Vec;

use nonos_eth_rpc::{parse_u64, request_estimate_gas};
use nonos_nox_config::{Contract, SETTLE_GAS_MAX};
use nonos_nox_meter::settle::settle_data;
use nonos_nox_meter::voucher::Voucher;
use nonos_payment_proto::{DrainSign, VoucherRecord};
use nonos_settle_proto::{padded, Settled, ENOENT, EPROTO};

use super::meter;
use super::owed::owed;
use crate::handlers::{quote_gas, send_raw};
use crate::settle::Settle;

fn voucher(r: &VoucherRecord) -> (Voucher, [u8; 65]) {
    let v = Voucher {
        channel_id: r.channel_id,
        payer: r.payer,
        payee: r.payee,
        listing_id: r.listing_id,
        unit: r.unit,
        cumulative: r.cumulative,
        seq: r.seq,
        expiry: r.expiry,
        usage_root: r.usage_root,
    };
    (v, r.sig)
}

pub fn round(s: &mut Settle, chain_id: u64) -> Result<Settled, i32> {
    let settlement = s.live.usable(&mut s.rpc, chain_id, Contract::Settlement)?;
    let due = owed(s, chain_id, &settlement, &meter::vouchers()?)?;
    let payer = due.first().ok_or(ENOENT)?.payer;
    let g = quote_gas(s, chain_id, &payer)?;
    let data = settle_data(&due.iter().map(voucher).collect::<Vec<_>>());
    let ask = request_estimate_gas(&payer, &settlement, 0, &data, 1);
    let estimate = parse_u64(&s.rpc.call(chain_id, &ask)?).ok_or(EPROTO)?;
    let sign = DrainSign {
        chain_id,
        nonce: g.tx_nonce,
        gas_limit: padded(estimate).ok_or(EPROTO)?.min(SETTLE_GAS_MAX),
        max_fee: g.max_fee_per_gas,
        max_priority: g.max_priority_fee_per_gas,
        channels: due.iter().map(|r| r.channel_id).collect(),
    };
    let tx_hash = send_raw(s, chain_id, &meter::sign(&sign)?)?;
    Ok(Settled { tx_hash, carried: due.iter().map(|r| (r.channel_id, r.cumulative)).collect() })
}
