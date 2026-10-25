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

//! The Shield send screen's amount checks, and which screens read the typed amount.

use crate::shield_typed::{amount_ok, reads_typed};

#[test]
fn amounts_are_positive_decimals_of_at_most_18_places() {
    for ok in ["1", "0.02", ".5", "5.", "2000", "0.000000000000000001"] {
        assert!(amount_ok(ok), "{ok} refused");
    }
    for bad in [
        "",
        ".",
        "0",
        "0.0",
        "00.000",
        "1.2.3",
        "-1",
        "1e3",
        "1,000",
        " 1",
        "0.0000000000000000001",
    ] {
        assert!(!amount_ok(bad), "{bad} accepted");
    }
}

/* The screens as state/shield_ui.rs numbers them. */
const DEPOSIT: u8 = 1;
const SEND: u8 = 2;
const WITHDRAW: u8 = 3;
const REVIEW: u8 = 4;

#[test]
fn only_a_send_or_its_review_reads_the_typed_amount() {
    assert!(reads_typed(SEND, 0, SEND, REVIEW));
    assert!(reads_typed(REVIEW, SEND, SEND, REVIEW));
    assert!(!reads_typed(REVIEW, DEPOSIT, SEND, REVIEW));
    assert!(!reads_typed(REVIEW, WITHDRAW, SEND, REVIEW));
    /* A send quoted before: the screen acted on now is a deposit. */
    assert!(!reads_typed(DEPOSIT, SEND, SEND, REVIEW));
    assert!(!reads_typed(WITHDRAW, SEND, SEND, REVIEW));
}
