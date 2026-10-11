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

//! OPEN's two calls, START and CONFIRM (see `open`).

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};
use nonos_nox_meter::open::{confirmed, open_plan};
use nonos_payment_proto::{OpenConfirm, OpenStart, ENOENT, ETIME};

use crate::clients::{broadcast, gas, opened};
use crate::keccak::Host;
use crate::keyring::sign_tx;
use crate::meter::{now_ms, Meter};
use crate::server::status_of;

fn contracts(chain_id: u64) -> Result<([u8; 20], [u8; 20]), i32> {
    let nox = address(chain_id, Contract::NoxToken).map_err(|_| ENOENT)?;
    let settlement = address(chain_id, Contract::Settlement).map_err(|_| ENOENT)?;
    Ok((nox, settlement))
}

pub(super) fn start(meter: &mut Meter, s: &OpenStart) -> Result<Vec<u8>, i32> {
    let keys = meter.ready()?;
    let (nox, settlement) = contracts(s.chain_id)?;
    if s.expiry <= now_ms() / 1000 {
        return Err(ETIME);
    }
    let g = gas(s.chain_id, &keys.address)?;
    let plan = open_plan(s, &nox, &settlement, &g).map_err(status_of)?;
    let mut out = Vec::with_capacity(64);
    for tx in [&plan.approve, &plan.open] {
        let raw = sign_tx(keys.key_id, tx)?;
        out.extend_from_slice(&broadcast(s.chain_id, &raw)?);
    }
    Ok(out)
}

pub(super) fn confirm(meter: &mut Meter, c: &OpenConfirm) -> Result<Vec<u8>, i32> {
    let keys = meter.ready()?;
    let (_, settlement) = contracts(c.start.chain_id)?;
    let ev = opened(c.start.chain_id, &c.open_tx)?;
    let channel = confirmed(&Host, &c.start, &keys.address, &settlement, &ev).map_err(status_of)?;
    let next = meter.state.with_channel(channel, now_ms() / 1000).map_err(status_of)?;
    meter.commit(next)?;
    Ok(channel.id.to_vec())
}
