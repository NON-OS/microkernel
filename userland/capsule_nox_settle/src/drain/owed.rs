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

//! Which vouchers a settle() should carry on one chain: those whose channel
//! the config Settlement holds there, not closed, with settled below the
//! voucher's cumulative. One batch reads deposited, settled and closed of
//! every channel; a channel of another chain reads as nothing deposited. A
//! voucher the contract skipped (VoucherSkipped) never moved settled, so it
//! is owed again.

use alloc::vec::Vec;

use nonos_payment_proto::VoucherRecord;
use nonos_settle_proto::{view, view_calldata, EINVAL, EPROTO, SETTLED_MAX};

use crate::chain::{eth_calls, word_bool, word_u128};
use crate::settle::Settle;

pub fn owed(
    s: &mut Settle,
    chain_id: u64,
    settlement: &[u8; 20],
    all: &[VoucherRecord],
) -> Result<Vec<VoucherRecord>, i32> {
    let data = |v, id: &[u8; 32]| view(v).and_then(|x| view_calldata(x, &[*id])).ok_or(EINVAL);
    let mut calls = Vec::with_capacity(all.len().saturating_mul(3));
    for r in all {
        for v in [10, 11, 13] {
            calls.push((*settlement, data(v, &r.channel_id)?));
        }
    }
    if calls.is_empty() {
        return Ok(Vec::new());
    }
    let words = eth_calls(&mut s.rpc, chain_id, &calls)?;
    let mut out = Vec::new();
    for (r, w) in all.iter().zip(words.chunks_exact(3)) {
        let [deposited, settled, closed] = w else { return Err(EPROTO) };
        let deposited = word_u128(deposited).ok_or(EPROTO)?;
        let settled = word_u128(settled).ok_or(EPROTO)?;
        let closed = word_bool(closed).ok_or(EPROTO)?;
        if deposited > 0 && !closed && r.cumulative > settled && out.len() < SETTLED_MAX {
            out.push(*r);
        }
    }
    Ok(out)
}
