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

//! Every press that moves value or signs is held to the arming Buy has
//! (the lead's ruling 1); every other press acts at once.

use crate::wallet::arm::moves::{moves, Shown};
use crate::wallet::arm::rows::{ROW_ATTEST, ROW_BINDING, ROW_CLAIM, ROW_CLOSE, ROW_TOPUP};
use crate::wallet::screen::hits::Press;

const ENTER: Option<Press> = None;

fn at(p: Press) -> Option<Press> {
    Some(p)
}

#[test]
fn every_press_that_moves_value_or_signs_is_guarded() {
    use Press::{Footer, Row};
    let guarded = [
        (Shown::Consent, at(Footer(1)), "Buy, Open channel or Use it"),
        (Shown::Consent, ENTER, "Enter on the voucher's channel form"),
        (Shown::Budget, at(Footer(1)), "Set budget for the channel"),
        (Shown::Escrow, at(Footer(1)), "Dispute, Refund or Try again"),
        (Shown::PayReview, at(Footer(0)), "a payment's Confirm"),
        (Shown::StakeReview, at(Footer(0)), "a staking Confirm"),
        (Shown::ShieldReview, at(Footer(0)), "a Shield deposit, send or withdraw"),
        (Shown::TakeBack(2), at(Footer(2)), "Take back"),
        (Shown::Landed(12), at(Row(12)), "a landed row's approval or deposits"),
        (Shown::Meter, at(Footer(1)), "the meter's Set budget"),
        (Shown::Meter, ENTER, "Enter: set budget or top up"),
        (Shown::Meter, at(Row(ROW_TOPUP)), "Top up"),
        (Shown::Meter, at(Row(ROW_CLOSE)), "Close the channel"),
        (Shown::Meter, at(Row(ROW_CLAIM)), "Claim"),
        (Shown::Publish, at(Row(ROW_BINDING)), "sign the binding"),
        (Shown::Publish, at(Row(ROW_ATTEST)), "sign the first attest"),
        (Shown::Publish, at(Row(ROW_ATTEST + 3)), "sign a later attest"),
        (Shown::Export, at(Footer(0)), "Show the key"),
    ];
    for (shown, press, what) in guarded {
        assert!(moves(shown, press), "{what} on {shown:?} must wait to be armed");
    }
}
