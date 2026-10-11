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

/* Why a purchase's job did not go, in words a person can act on. */

use nonos_payment_proto::{EBUSY, EINVAL, ENOENT, ENOPERSIST, EPROTO, ETIME};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// ORDER_OFFER: the proof of work and the seller agent's sealed answer.
    Offer,
    /// ORDER_NEW: the terms, checked against the chain.
    Order,
}

pub fn failed(stage: Stage, status: i32) -> &'static str {
    match (stage, status) {
        (Stage::Offer, ETIME) => "The seller's agent could not be asked in time; try again.",
        (Stage::Offer, EINVAL) => "This package is too large for an escrow purchase, or its seller's address is not https.",
        (Stage::Offer, EPROTO) => "The seller's agent answered with an offer that does not open; nothing was ordered.",
        (Stage::Order, ENOENT) => "The meter no longer holds this offer, or the escrow is not on this chain; start again.",
        (Stage::Order, ETIME) => "The delivery deadline fell outside the escrow's window; start again.",
        (Stage::Order, EPROTO) => "The chain did not agree with the meter's deposit address; nothing was ordered.",
        (Stage::Order, EBUSY) => "The escrow is paused, or the meter is busy; try again later.",
        (_, EBUSY) => "The meter is busy with another job; try again soon.",
        (_, ENOPERSIST) => "The meter has no disk to keep the order on.",
        (_, ENOENT) => "The meter restarted, so this job's answer is gone; start again.",
        _ => "The meter could not make this order.",
    }
}
