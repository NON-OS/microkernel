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

//! ORDER_NEW, ORDER_STATUS and ORDER_REFUND. NEW plans
//! the order under fresh salts and runs it as a job that reads paused() and
//! depositAddress, then seals it before answering; STATUS is a job over a
//! kept order; REFUND (refund.rs) answers the wallet the refund pair.

use alloc::vec::Vec;

use nonos_nox_config::{address, forwarder_initcode_hash, Contract};
use nonos_nox_meter::flow::Work as Job;
use nonos_nox_meter::orders::{order_plan, Order};
use nonos_payment_proto::{OrderNew, EINVAL, ENOENT};

use super::channel_ops::started;
use crate::jobs::Work;
use crate::keccak::Host;
use crate::keys::random;
use crate::meter::{now_ms, Meter};
use crate::orders::find;
use crate::server::{status_of, Call};
use crate::wipe::wipe;

pub fn order_new(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let job = OrderNew::parse(req.body).ok_or(EINVAL).and_then(|n| planned(&n));
    started(work, meter, job.map(Job::OrderNew), req)
}

fn planned(n: &OrderNew) -> Result<Order, i32> {
    let at = |c| address(n.chain_id, c).map_err(|_| ENOENT);
    let (nox, escrow) = (at(Contract::NoxToken)?, at(Contract::Escrow)?);
    let fwd = forwarder_initcode_hash(n.chain_id).ok_or(ENOENT)?;
    let mut salts = [0u8; 64];
    random(&mut salts)?;
    let (mut salt, mut refund) = ([0u8; 32], [0u8; 32]);
    salt.copy_from_slice(&salts[..32]);
    refund.copy_from_slice(&salts[32..]);
    wipe(&mut salts);
    let o = order_plan(&Host, n, (&nox, &escrow, &fwd), (salt, refund), now_ms() / 1000);
    wipe(&mut salt);
    wipe(&mut refund);
    o.map_err(status_of)
}

pub(super) fn order_id(req: &Call<'_>) -> Result<[u8; 32], i32> {
    req.body.try_into().map_err(|_| EINVAL)
}

pub fn order_status(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    let job = order_id(req).and_then(|id| find(&id));
    started(work, meter, job.map(Job::OrderStatus), req)
}
