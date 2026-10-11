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

//! What each step of a purchase says first, and which footer it offers.

use crate::wallet::order::fees::Fees;
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;

pub const CONSENT: &str = "The store asks to buy this. What follows is the market's own record, \
     not the store's: buy only if it is what you meant.";
const QUOTING: &str = "Asking the Shield what funding this price costs. It decides whether \
     the price is one of its standard amounts today.";
const REFUSED: &str = "The Shield will not fund this price, so it cannot be paid privately, \
     and this wallet does not pay escrow from a plain address yet.";
const OFFERING: &str = "The meter is asking the seller's agent for its sealed offer. Its proof \
     of work runs between other payments, so this can take a minute.";
const ORDERING: &str = "The meter is making the order and checking its deposit address against \
     the chain.";
const FUND: &str = "The order is made. Fund it from the Shield: send exactly the price, in NOX, \
     to its deposit address. If the seller does not deliver, it comes back to you.";

/// The lead sentence, and the action offered beside Close, if any.
pub fn lead(p: &Pending) -> (&'static str, Option<&'static str>) {
    match (&p.step, &p.read) {
        (Step::Consent, Ok(Named::Escrow(_))) => match p.fees {
            Fees::Quoted(_) => (CONSENT, Some("Buy")),
            Fees::Refused(_) => (REFUSED, None),
            Fees::Unasked | Fees::Asking => (QUOTING, None),
        },
        (Step::Consent, Err(why)) => (why, None),
        (Step::Offering(_), _) => (OFFERING, None),
        (Step::Ordering(..), _) => (ORDERING, None),
        (Step::Fund(_), _) => {
            super::escrow_words::lead(p).unwrap_or((FUND, Some("Fund from the Shield")))
        }
        (Step::Consent, Ok(Named::Voucher(_))) => super::words_voucher::consent(p.held),
        (Step::Opening(..) | Step::Confirming(..) | Step::Opened(_) | Step::Kept(_), _) => {
            super::words_voucher::lead(p)
        }
        (Step::Failed(why), _) => (why, None),
    }
}

/// Whether the person may close the screen: not while a job runs, nor
/// while a mined opening still waits for the meter to record it.
pub fn closable(p: &Pending) -> bool {
    use crate::wallet::order::send_step::SendStep::{Broadcasting, Signing};
    let sending = p.escrow.as_ref().is_some_and(|e| matches!(e.step, Signing(_) | Broadcasting(_)));
    !sending
        && !matches!(
            p.step,
            Step::Offering(_) | Step::Ordering(..) | Step::Opening(..) | Step::Confirming(..)
        )
}
