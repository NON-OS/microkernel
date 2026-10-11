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

/*
 * What funding an order costs, asked of the shield service at consent (B8,
 * the plan): its own rules decide whether the price is a standard amount on
 * today's range, and its quote of the withdraw fee shows before Buy. The
 * amount is written exactly, never rounded.
 */

use shield_wire::{field, OP_QUOTE};

pub use super::fees_kind::{exact, Fees};
use super::intake::Step;
use super::read::Named;
use crate::wallet::shield::{actions::coin, job, open};
use crate::wallet::state::shield_ui::ASSET_NOX;
use crate::wallet::state::State;

const NO_QUOTE: &str = "The shield service gave no quote for this price.";

/// Ask the quote for an escrow price at consent; whether anything changed.
pub fn ask(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref() else { return false };
    let (Step::Consent, Ok(Named::Escrow(b)), Fees::Unasked) = (&p.step, &p.read, &p.fees) else {
        return false;
    };
    if open::here() && state.shield_ui.waiting.is_some() {
        return false;
    }
    let amount = exact(b.amount);
    let fees = match job::start(state, OP_QUOTE, &[coin(ASSET_NOX), &amount, "1"]) {
        true => Fees::Asking,
        false => Fees::Refused(state.shield_ui.failure.take().unwrap_or_else(|| NO_QUOTE.into())),
    };
    state.meter.purchase.as_deref_mut().map(|p| p.fees = fees).is_some()
}

/// Whether a quote now landing is this purchase's: then it is kept here,
/// and the shield's own screens are left as they were.
pub fn quoted(state: &mut State, body: &str) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut().filter(|p| matches!(p.fees, Fees::Asking))
    else {
        return false;
    };
    p.fees = match field(body, "refusal") {
        Some(why) => Fees::Refused(why.into()),
        None => Fees::Quoted(field(body, "total_fee").unwrap_or("not reported").into()),
    };
    true
}

/// A quote job that ended without one: its failure is this purchase's.
pub fn settled(state: &mut State) -> bool {
    let asking = state.meter.purchase.as_deref().is_some_and(|p| matches!(p.fees, Fees::Asking));
    if !asking || state.shield_ui.waiting.is_some() {
        return false;
    }
    let why = state.shield_ui.failure.take().unwrap_or_else(|| NO_QUOTE.into());
    state.meter.purchase.as_deref_mut().map(|p| p.fees = Fees::Refused(why)).is_some()
}
