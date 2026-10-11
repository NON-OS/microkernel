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

//! TOPUP (op 13) and CLOSE (op 14), from the wallet. The
//! meter asks settle for its gas, encodes and signs with its own key, and has
//! settle broadcast; it answers the transaction hashes. Neither changes the
//! meter's state: the next CHANNEL_STATE reads the new deposit, or the close.

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::settle::{close_plan, topup_plan};
use nonos_payment_proto::{Close, TopUp, EINVAL, EIO, ENOENT};

use crate::clients::{broadcast, gas, read, word_u64};
use crate::keyring::sign_tx;
use crate::meter::{now_ms, Meter};
use crate::server::{status_of, Call};

/// Settlement's GRACE() (settle_proto view 16).
const VIEW_GRACE: u16 = 16;

fn contract(chain_id: u64, c: Contract) -> Result<[u8; 20], i32> {
    address(chain_id, c).map_err(|_| ENOENT)
}

pub fn topup(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    req.result(run_topup(meter, req.body))
}

pub fn close(meter: &mut Meter, req: &Call<'_>) -> Vec<u8> {
    req.result(run_close(meter, req.body))
}

fn run_topup(meter: &mut Meter, body: &[u8]) -> Result<Vec<u8>, i32> {
    let t = TopUp::parse(body).ok_or(EINVAL)?;
    let k = meter.ready()?;
    let nox = contract(t.chain_id, Contract::NoxToken)?;
    let settlement = contract(t.chain_id, Contract::Settlement)?;
    let g = gas(t.chain_id, &k.address)?;
    let now = now_ms() / 1000;
    let txs = topup_plan(&meter.state, &t, (&nox, &settlement), &g, now).map_err(status_of)?;
    let mut out = Vec::with_capacity(64);
    for tx in &txs {
        out.extend_from_slice(&broadcast(t.chain_id, &sign_tx(k.key_id, tx)?)?);
    }
    Ok(out)
}

fn run_close(meter: &mut Meter, body: &[u8]) -> Result<Vec<u8>, i32> {
    let c = Close::parse(body).ok_or(EINVAL)?;
    let k = meter.ready()?;
    let settlement = contract(c.chain_id, Contract::Settlement)?;
    let grace = read(c.chain_id, VIEW_GRACE, &[], 1)?;
    let grace = grace.first().and_then(word_u64).ok_or(EIO)?;
    let g = gas(c.chain_id, &k.address)?;
    let now = now_ms() / 1000;
    let tx = close_plan(&meter.state, &c, &settlement, &g, (now, grace)).map_err(status_of)?;
    Ok(broadcast(c.chain_id, &sign_tx(k.key_id, &tx)?)?.to_vec())
}
