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
//! A retry that never sends twice (v1.4.55). A broadcast made before goes
//! again as the same raw, never re-signed over a new nonce; a Shield
//! withdraw for the gas is waited on, not offered again, until it fails or
//! GAS_WAIT_S pass. Pure.

use alloc::vec::Vec;

use super::escrow_offer::Offer;
use super::send_step::SendStep;

/// How long a started gas withdraw holds the button, in seconds.
pub const GAS_WAIT_S: u64 = 600;

/// The step after a retry's orderOf read: `next` as read, unless a raw was
/// broadcast before; then Done if the call landed, the read's refusal if
/// the escrow no longer allows it, and otherwise that same raw again.
pub fn resume(next: SendStep, raw: Option<&Vec<u8>>, landed: bool) -> SendStep {
    match (raw, next) {
        (None, next) => next,
        (Some(_), _) if landed => SendStep::Done,
        (Some(_), failed @ SendStep::Failed(_)) => failed,
        (Some(raw), _) => SendStep::Broadcasting(raw.clone()),
    }
}

/// The offer, with PayGas held while a withdraw paid at `paid_s` may land.
pub fn gas_offer(o: Option<Offer>, paid_s: Option<u64>, now_s: u64) -> Option<Offer> {
    match (o, paid_s) {
        (Some(Offer::PayGas), Some(t)) if now_s < t.saturating_add(GAS_WAIT_S) => {
            Some(Offer::GasWaiting)
        }
        (o, _) => o,
    }
}
