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

use crate::wallet::order::escrow_offer::{offer, Offer};
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::refund::deadline;
use crate::wallet::order::send_step::{SendStep, DISPUTE};

const BAD: &str = "A chunk of the delivery is not what was listed. Dispute it from this \
     wallet's refund address before the window ends: the seller's bond answers for it.";
const REFUNDABLE: &str = "The escrow allows a refund now. It goes to this wallet's refund \
     address, and only this machine can send it: the order's record is kept here.";
const READING: &str = "Reading the order from the escrow.";
const QUOTING: &str = "Asking settle what the transaction costs, from the refund address.";
const GAS: &str = "The refund address needs ETH for the gas. Pay it from the Shield: a \
     standard amount, sent privately; what is left stays at that address.";
const SIGNING: &str = "The Shield is signing from the refund address.";
const SENDING: &str = "Sending the transaction through settle.";
const SENT: &str = "Sent. Waiting for the escrow to record it.";
const DISPUTED: &str = "Disputed: the seller's bond answers for it. A refund opens now.";
const REFUNDED: &str = "Refunded to this wallet's refund address.";

/// The made order's offer, now.
pub fn offered(p: &Pending) -> Option<Offer> {
    let Step::Fund(m) = &p.step else { return None };
    let step = p.escrow.as_ref().map(|e| &e.step);
    let due = deadline(m).unwrap_or(u64::MAX);
    offer(step, p.watch.status.as_ref(), p.delivery.made.as_ref(), due, nonos_tls::rtc_now())
}

/// the project sentence and action while a dispute or refund is offered or
/// under way; None leaves the funding step's own.
pub fn lead(p: &Pending) -> Option<(&'static str, Option<&'static str>)> {
    let label = offered(p).map(|o| match o {
        Offer::Dispute => "Dispute",
        Offer::Refund => "Refund",
        Offer::PayGas => "Pay the gas from the Shield",
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
