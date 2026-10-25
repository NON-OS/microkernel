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

//! The transactions each job signs, from settle's GAS and its one read:
//! GRACE for CLOSE, what is claimable for CLAIM.

use alloc::vec::Vec;

use nonos_settle_proto::Gas;

use super::ask::Contracts;
use super::words::first_u128;
use super::work::Work;
use crate::error::MeterError;
use crate::open::{open_plan, TxRequest};
use crate::settle::{claim_plan, close_plan, topup_plan};
use crate::state::MeterState;

pub(super) fn txs(
    work: &Work,
    got: &[Vec<[u8; 32]>],
    g: &Gas,
    s: &MeterState,
    c: &Contracts,
    now: u64,
) -> Result<Vec<TxRequest>, MeterError> {
    Ok(match work {
        Work::Open(o) if o.expiry <= now => return Err(MeterError::Expired),
        Work::Open(o) => {
            let p = open_plan(o, &c.nox, &c.settlement, g)?;
            Vec::from([p.approve, p.open])
        }
        Work::TopUp(t) => Vec::from(topup_plan(s, t, (&c.nox, &c.settlement), g, now)?),
        Work::Close(cl) => {
            let grace = first_u128(got.first())
                .and_then(|r| u64::try_from(r).ok())
                .ok_or(MeterError::TooEarly)?;
            Vec::from([close_plan(s, cl, &c.settlement, g, (now, grace))?])
        }
        Work::OrderNew(_) | Work::OrderStatus(_) | Work::Offer(_) | Work::Deliver(_) => {
            return Err(MeterError::Mismatch)
        }
        Work::Claim { chain_id, meter } => Vec::from([claim_plan(
            *chain_id,
            meter,
            &c.pool,
            first_u128(got.first()).unwrap_or(0),
            g,
        )?]),
    })
}
