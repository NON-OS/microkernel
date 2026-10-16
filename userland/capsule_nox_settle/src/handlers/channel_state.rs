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

//! CHANNEL_STATE: deposited, settled, expiryOf and closed of one channel, in
//! one batch, and settled again from a second host. Two hosts that disagree
//! on settled are EIO: the meter freezes on settled, so one host must not
//! decide it alone. A voucher the contract skipped (VoucherSkipped) never
//! moved settled, so it reads as unsettled here.

use alloc::vec::Vec;

use nonos_nox_config::Contract;
use nonos_settle_proto::{
    parse_chain_hash, view, view_calldata, ChannelState, EINVAL, EIO, EPROTO,
};

use crate::chain::{eth_calls, eth_calls_at, word_bool, word_u128, word_u64};
use crate::settle::Settle;

pub fn channel_state(s: &mut Settle, body: &[u8]) -> Result<Vec<u8>, i32> {
    let ch = parse_chain_hash(body).ok_or(EINVAL)?;
    let at = s.live.usable(&mut s.rpc, ch.chain_id, Contract::Settlement)?;
    let data = |v| view(v).and_then(|x| view_calldata(x, &[ch.id])).ok_or(EINVAL);
    let calls = [(at, data(10)?), (at, data(11)?), (at, data(12)?), (at, data(13)?)];
    let r = eth_calls(&mut s.rpc, ch.chain_id, &calls)?;
    let [deposited, settled, expiry, closed] = r.as_slice() else { return Err(EPROTO) };
    let again = eth_calls_at(&mut s.rpc, ch.chain_id, &calls[1..2], true)?;
    let [again] = again.as_slice() else { return Err(EPROTO) };
    if again != settled {
        return Err(EIO);
    }
    let state = ChannelState {
        deposited: word_u128(deposited).ok_or(EPROTO)?,
        settled: word_u128(settled).ok_or(EPROTO)?,
        expiry: word_u64(expiry).ok_or(EPROTO)?,
        closed: word_bool(closed).ok_or(EPROTO)?,
    };
    Ok(state.encode().to_vec())
}
