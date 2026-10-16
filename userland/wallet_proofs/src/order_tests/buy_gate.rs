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

//! Buy's gate (B8, the plan): Buy acts only at consent, on terms the market
//! read as escrow, once the Shield quoted their price; the quote is asked
//! once, at consent, for that price written exactly. Nothing else moves.

use super::fixture::buy;
use super::gate_fixture::{fees, reads, steps};
use crate::order::buy_gate::{buyable, to_quote};
use crate::order::fees_kind::{exact, Fees};
use crate::order::pending_step::Step;
use crate::order::read::Named;

#[test]
fn buy_acts_only_at_consent_on_escrow_terms_the_shield_quoted() {
    let mut acted = 0;
    for (i, step) in steps().iter().enumerate() {
        for read in &reads() {
            for fee in &fees() {
                let got = buyable(step, read, fee);
                let at_consent = matches!(step, Step::Consent);
                let escrow = matches!(read, Ok(Named::Escrow(_)));
                if at_consent && escrow && matches!(fee, Fees::Quoted(_)) {
                    assert_eq!(got, Some(&buy()), "the market's terms, untouched");
                    acted += 1;
                } else {
                    assert_eq!(got, None, "step {i}, read {read:?}");
                }
            }
        }
    }
    assert_eq!(acted, 1, "one combination of the 144 acts");
}

#[test]
fn the_quote_is_asked_once_at_consent_for_the_blocks_exact_price() {
    let mut asked = 0;
    for (i, step) in steps().iter().enumerate() {
        for read in &reads() {
            for (f, fee) in fees().iter().enumerate() {
                let got = to_quote(step, read, fee);
                let at_consent = matches!(step, Step::Consent);
                let escrow = matches!(read, Ok(Named::Escrow(_)));
                if at_consent && escrow && matches!(fee, Fees::Unasked) {
                    assert_eq!(got, Some(exact(buy().amount)));
                    asked += 1;
                } else {
                    assert_eq!(got, None, "step {i}, read {read:?}, fees {f}");
                }
            }
        }
    }
    assert_eq!(asked, 1);
}
