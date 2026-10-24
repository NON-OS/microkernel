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

//! Which presses move value or sign (the the maintainers' ruling 1, 2026-10-17), each held
//! to the arming escrow Buy has (order/terms/arming.rs): the screen drawn whole,
//! the wallet focused, held a second, asked again at the press. Every other press,
//! and every key but Enter, reads, reviews, chooses, copies or leaves at once.

use super::rows::{ROW_ATTEST, ROW_BINDING, ROW_CLAIM, ROW_CLOSE, ROW_INTENT, ROW_TOPUP};
use crate::wallet::screen::hits::Press;

/// The one screen at a time whose presses the guard looks at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shown {
    /// A purchase's consent, escrow or voucher: Buy, Open channel or Use
    /// it, and Enter on the channel's form. Its guard is the purchase's
    /// own (order/guard), which reads the terms again on focus lost.
    Consent,
    /// An open voucher channel the meter has no budget for: Set budget.
    Budget,
    /// A made order offering Dispute, Refund or Try again, each signed
    /// from refundTo and sent once pressed.
    Escrow,
    /// A payment's review: Confirm signs and sends it.
    PayReview,
    /// A staking review: Confirm signs the approve, stake or unstake.
    StakeReview,
    /// A Shield review: Confirm deposits, sends, withdraws or settles.
    ShieldReview,
    /// The Proving screen, Take back at this footer.
    TakeBack(u8),
    /// Shield home, this landed row offering its approval or deposits.
    Landed(u8),
    /// The meter: Set budget (and Enter), Top up, Close, Claim.
    Meter,
    /// An entry document open to publish: the binding and attests signed.
    Publish,
    /// A publisher transaction's review: Sign and send signs it.
    PublisherReview,
    /// A release's attest under review: Sign the attest signs it.
    AttestReview,
    /// The private key asked for, still hidden: Show the key.
    Export,
}

/// Whether `press` on `shown` moves value or signs; None is Enter.
pub fn moves(shown: Shown, press: Option<Press>) -> bool {
    use Shown::*;
    match (shown, press) {
        (Consent | Meter, Some(Press::Footer(1)) | None) => true,
        (Budget | Escrow, Some(Press::Footer(1))) => true,
        (
            PayReview | StakeReview | ShieldReview | PublisherReview | AttestReview | Export,
            Some(Press::Footer(0)),
        ) => true,
        (TakeBack(at), Some(Press::Footer(n))) => n == at,
        (Landed(at), Some(Press::Row(r))) => r == at,
        (Meter, Some(Press::Row(r))) => matches!(r, ROW_TOPUP | ROW_CLOSE | ROW_CLAIM),
        (Publish, Some(Press::Row(r))) => r == ROW_BINDING || (ROW_ATTEST..ROW_INTENT).contains(&r),
        _ => false,
    }
}
