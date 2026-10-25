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

//! What each job reads before anything else, view by view (READ), and its
//! first ask. CLOSE reads GRACE; CLAIM what the pool credits the meter;
//! ORDER_NEW the policy's paused() where there is one, then the escrow's
//! depositAddress(orderId); ORDER_STATUS orderOf, NOX at the deposit
//! address, and what the pool credits it where there is a pool.

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract};

use super::ask::Ask;
use super::words::word;
use super::work::Work;
use crate::settle::NOX_ASSET;

impl Work {
    pub fn reads(&self) -> Vec<(u16, Vec<[u8; 32]>)> {
        let has = |c| address(self.chain_id(), c).is_ok();
        let nox = word(&NOX_ASSET.to_be_bytes());
        let mut out = Vec::new();
        match self {
            Work::Open(_) | Work::TopUp(_) | Work::Offer(_) | Work::Deliver(_) => {}
            Work::Close(_) => out.push((16, Vec::new())),
            Work::Claim { meter, .. } => out.push((60, Vec::from([nox, word(meter)]))),
            Work::OrderNew(o) => {
                if has(Contract::AmountPolicy) {
                    out.push((72, Vec::new()));
                }
                out.push((21, Vec::from([o.order_id])));
            }
            Work::OrderStatus(o) => {
                out.push((20, Vec::from([o.order_id])));
                out.push((1, Vec::from([word(&o.deposit)])));
                if has(Contract::ShieldPool) {
                    out.push((60, Vec::from([nox, word(&o.deposit)])));
                }
            }
        }
        out
    }

    /// The first ask: the first read, or GAS for a job that reads nothing.
    pub fn first(&self) -> Ask {
        match self {
            Work::Offer(w) => return Ask::Fetch(w.request.clone()),
            Work::Deliver(o) => return Ask::Delivered(o.order_id),
            _ => {}
        }
        match self.reads().into_iter().next() {
            Some((view, args)) => Ask::View { view, args },
            None => Ask::Gas,
        }
    }
}
