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
 * When an escrow purchase may be quoted and bought (B8, plan/27): at
 * consent, read from the market as escrow; the Shield is asked its quote
 * once, for the block's price written exactly, and Buy acts only on a
 * price it quoted with a fee that reads as NOX. Anything else does
 * nothing; whether Buy is armed on screen is arming.rs.
 */

use alloc::string::String;

use super::buy::Buy;
use super::fees_kind::{exact, fee_wei, Fees};
use super::pending_step::Step;
use super::read::Named;

/// The escrow purchase Buy acts on, if this is one it may.
pub fn buyable<'a>(
    step: &Step,
    read: &'a Result<Named, &'static str>,
    fees: &Fees,
) -> Option<&'a Buy> {
    let (Step::Consent, Ok(Named::Escrow(buy)), Fees::Quoted(fee)) = (step, read, fees) else {
        return None;
    };
    fee_wei(fee).map(|_| buy)
}

/// The amount the Shield is asked to quote, if one is to be asked now.
pub fn to_quote(step: &Step, read: &Result<Named, &'static str>, fees: &Fees) -> Option<String> {
    let (Step::Consent, Ok(Named::Escrow(b)), Fees::Unasked) = (step, read, fees) else {
        return None;
    };
    Some(exact(b.amount))
}
