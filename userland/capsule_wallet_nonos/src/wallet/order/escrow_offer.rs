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

//! What a made order offers beside Close (B8, v1.4.51): a dispute for a bad
//! chunk inside its window, a refund once the escrow allows one, the gas for
//! a send under way, or a send again after one failed. NOX waiting at the
//! deposit address counts in every state: the refund sweeps it (v1.4.55). Pure.

use nonos_payment_proto::{DeliveredMade, OrderStatus};

use super::send_step::SendStep;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Offer {
    Dispute,
    Refund,
    PayGas,
    /// A Shield withdraw for the gas is under way: nothing to press.
    GasWaiting,
    Again,
}

/// `step` is a send under way, if any; `deadline` the order's own.
pub fn offer(
    step: Option<&SendStep>,
    status: Option<&OrderStatus>,
    delivered: Option<&DeliveredMade>,
    deadline: u64,
    now_s: u64,
) -> Option<Offer> {
    match step {
        Some(SendStep::Gas { .. }) => return Some(Offer::PayGas),
        Some(SendStep::Failed(_)) => return Some(Offer::Again),
        // A dispute done leaves the refund it opens to be offered.
        Some(SendStep::Done) | None => {}
        Some(_) => return None,
    }
    let s = status?;
    let bad = matches!(delivered, Some(DeliveredMade::BadChunk { .. }));
    match s.state {
        2 if bad => Some(Offer::Dispute),
        3 => Some(Offer::Refund),
        1 if now_s >= deadline => Some(Offer::Refund),
        0 if s.pending > 0 && now_s >= deadline => Some(Offer::Refund),
        4 | 5 if s.pending > 0 => Some(Offer::Refund),
        _ => None,
    }
}
