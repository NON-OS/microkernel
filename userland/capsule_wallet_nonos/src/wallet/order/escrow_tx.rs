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

//! A dispute's or a refund's transaction through settle (v1.4.53): the
//! INTENT quote, then refundTo's ETH from GAS until the gas is there. Each
//! asked ASYNC, one at a time.

use nonos_nox_config::{address, Contract};
use nonos_settle_proto::{Gas, Intent, Quote, OP_GAS, OP_INTENT};

use super::escrowing::{Ctx, Escrowing};
use super::send_gas::{after_balance, after_quote};
use super::send_step::SendStep;
use crate::wallet::settle::{SettleCall, ESCROW};

pub(super) const GAS_EVERY_MS: u64 = 10_000;
const NO_QUOTE: &str = "settle did not quote the transaction; try again.";

/// INTENT for the send, then its quote held to the kind's bounds.
pub fn quoting(settle: &mut SettleCall, e: &mut Escrowing, c: &Ctx) -> bool {
    let SendStep::Quoting(send) = &e.step else { return false };
    let Some(r) = settle.take(OP_INTENT, ESCROW) else {
        let (kind, from, calldata) = (send.kind, send.from, send.calldata.clone());
        let i = Intent { chain_id: c.chain_id, kind, from, value: 0, calldata };
        let _ = settle.ask(OP_INTENT, ESCROW, &i.encode(), c.now_ms);
        return false;
    };
    let escrow = address(c.chain_id, Contract::Escrow).ok();
    e.step = match r.ok().and_then(|b| Quote::parse(&b)) {
        Some(q) => after_quote(send.clone(), q, escrow, 0, c.chain_id),
        None => SendStep::Failed(NO_QUOTE),
    };
    true
}

/// GAS for refundTo every GAS_EVERY_MS until it holds the gas's cost.
pub fn gas(settle: &mut SettleCall, e: &mut Escrowing, c: &Ctx) -> bool {
    let SendStep::Gas { send, .. } = &e.step else { return false };
    let Some(r) = settle.take(OP_GAS, ESCROW) else {
        if c.now_ms >= e.next_ms {
            e.next_ms = c.now_ms + GAS_EVERY_MS;
            let _ = settle.ask(OP_GAS, ESCROW, &Gas::request(c.chain_id, &send.from), c.now_ms);
        }
        return false;
    };
    let Some(balance) = r.ok().and_then(|b| Gas::parse(&b)).map(|g| g.balance) else {
        return false;
    };
    let step = core::mem::replace(&mut e.step, SendStep::Reading);
    e.step = after_balance(step, balance, c.chain_id);
    matches!(e.step, SendStep::Signing(_))
}
