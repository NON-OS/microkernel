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

//! A dispute or a refund under way for a purchase (B8, v1.4.51 to v1.4.53):
//! its kind, its step, and when settle is asked next.

use nonos_payment_proto::{DeliveredMade, OrderMade};

use super::send_step::SendStep;

pub struct Escrowing {
    pub kind: u8,
    pub step: SendStep,
    /// When settle is asked next for a read that repeats (orderOf, GAS).
    pub next_ms: u64,
    /// SIGN_FROM_FRESH was asked of the Shield for this signature.
    pub signing: bool,
    /// A dispute's window end, deliveredAt + disputeWindow, once read.
    pub window_end: Option<u64>,
}

impl Escrowing {
    pub fn new(kind: u8) -> Escrowing {
        Escrowing { kind, step: SendStep::Reading, next_ms: 0, signing: false, window_end: None }
    }
}

/// What a step reads of the purchase.
pub struct Ctx<'a> {
    pub chain_id: u64,
    pub made: &'a OrderMade,
    pub delivered: Option<&'a DeliveredMade>,
    pub refund_to: [u8; 20],
    /// NOX waiting at the deposit address, from ORDER_STATUS.
    pub pending: u128,
    pub now_ms: u64,
    /// Unix seconds, for the deadline and the window.
    pub now_s: u64,
}
