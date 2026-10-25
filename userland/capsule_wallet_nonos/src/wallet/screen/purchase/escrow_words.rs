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

//! What a made order's dispute or refund says first, and the action offered
//! beside Close (B8, v1.4.51 to v1.4.53).

use super::escrow_said::*;
use crate::wallet::order::escrow_offer::{offer, Offer};
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::refund::deadline;
use crate::wallet::order::send_resume::gas_offer;
use crate::wallet::order::send_step::{SendStep, DISPUTE};

/// The made order's offer, now.
pub fn offered(p: &Pending) -> Option<Offer> {
    let Step::Fund(m) = &p.step else { return None };
    let step = p.escrow.as_ref().map(|e| &e.step);
    let due = deadline(m).unwrap_or(u64::MAX);
    let now = crate::wallet::clock::now_s();
    let o = offer(step, p.watch.status.as_ref(), p.delivery.made.as_ref(), due, now);
    gas_offer(o, p.escrow.as_ref().and_then(|e| e.gas_paid_s), now)
}

/// the project sentence and action while a dispute or refund is offered or
/// under way; None leaves the funding step's own.
pub fn lead(p: &Pending) -> Option<(&'static str, Option<&'static str>)> {
    let label = offered(p).map(|o| match o {
        Offer::Dispute => "Dispute",
        Offer::Refund => "Refund",
        Offer::PayGas => "Pay the gas from the Shield",
        Offer::GasWaiting => "Waiting for the gas withdraw",
        Offer::Again => "Try again",
    });
    let Some(e) = p.escrow.as_ref() else {
        return match offered(p)? {
            Offer::Dispute => Some((BAD, label)),
            _ => Some((REFUNDABLE, label)),
        };
    };
    let said = match &e.step {
        SendStep::Reading | SendStep::RefundTo(_) => READING,
        SendStep::Quoting(_) => QUOTING,
        SendStep::Gas { .. } => GAS,
        SendStep::Signing(_) => SIGNING,
        SendStep::Broadcasting(_) => SENDING,
        SendStep::Sent(_) => SENT,
        SendStep::Done if e.kind == DISPUTE => DISPUTED,
        SendStep::Done => REFUNDED,
        SendStep::Failed(why) => why,
    };
    Some((said, label))
}
