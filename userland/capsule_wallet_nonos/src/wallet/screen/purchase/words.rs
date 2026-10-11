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

use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::screen::shield::sizes::sizes;
use crate::wallet::state::shield_ui::ASSET_NOX;

pub const CONSENT: &str = "The store asks to buy this. What follows is the market's own record, \
     not the store's: buy only if it is what you meant.";
const NOT_STANDARD: &str = "This price is not one of the Shield's standard amounts, so it \
     cannot be paid privately, and this wallet does not pay escrow from a plain address yet.";
const VOUCHER: &str = "This listing is paid by voucher as it is used: open a channel for it \
     from the Meter screen.";
const OFFERING: &str = "The meter is asking the seller's agent for its sealed offer. Its proof \
     of work runs between other payments, so this can take a minute.";
const ORDERING: &str = "The meter is making the order and checking its deposit address against \
     the chain.";
const FUND: &str = "The order is made. Fund it from the Shield: send exactly the price, in NOX, \
     to its deposit address. If the seller does not deliver, it comes back to you.";

/// Where the Shield's standard NOX sizes hold `amount`, if they do: the
/// only prices a withdrawal can fund.
pub fn standard(amount: u128) -> Option<u8> {
    sizes(ASSET_NOX).iter().position(|(_, v)| *v == amount).map(|i| i as u8)
}

/// The lead sentence, and the action offered beside Close, if any.
pub fn lead(p: &Pending) -> (&'static str, Option<&'static str>) {
    match (&p.step, &p.read) {
        (Step::Consent, Ok(Named::Escrow(b))) if standard(b.amount).is_none() => {
            (NOT_STANDARD, None)
        }
        (Step::Consent, Ok(Named::Escrow(_))) => (CONSENT, Some("Buy")),
        (Step::Consent, Ok(Named::Voucher(_))) => (VOUCHER, None),
        (Step::Consent, Err(why)) => (why, None),
        (Step::Offering(_), _) => (OFFERING, None),
        (Step::Ordering(..), _) => (ORDERING, None),
        (Step::Fund(_), _) => (FUND, Some("Fund from the Shield")),
        (Step::Failed(why), _) => (why, None),
    }
}

/// Whether the person may close the screen: not while a job runs.
pub fn closable(p: &Pending) -> bool {
    !matches!(p.step, Step::Offering(_) | Step::Ordering(..))
}
