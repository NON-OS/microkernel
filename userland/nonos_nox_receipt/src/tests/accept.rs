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

use super::fixtures::*;
use super::receipt::receipt;
use crate::verify::{verify_payment, Payment, ReceiptError};

#[test]
fn a_valid_payment_is_accepted() {
    let r = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE));
    let p = verify_payment(r.as_bytes(), &PAYEE, PRICE, &token()).expect("valid");
    assert_eq!(p, Payment { from: BUYER, amount: PRICE });
}

#[test]
fn overpayment_is_accepted_and_reported() {
    let r = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE * 2));
    let p = verify_payment(r.as_bytes(), &PAYEE, PRICE, &token()).expect("valid");
    assert_eq!(p.amount, PRICE * 2);
}

#[test]
fn an_underpayment_is_rejected() {
    let r = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE - 1));
    assert_eq!(
        verify_payment(r.as_bytes(), &PAYEE, PRICE, &token()),
        Err(ReceiptError::NoMatchingTransfer)
    );
    // The exact price is enough; underpayment is strictly below.
    let ok = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE));
    assert!(verify_payment(ok.as_bytes(), &PAYEE, PRICE, &token()).is_ok());
}

#[test]
fn a_payment_to_one_payee_does_not_pay_another() {
    let r = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE));
    let other = [0x5b; 20];
    assert_eq!(
        verify_payment(r.as_bytes(), &other, PRICE, &token()),
        Err(ReceiptError::NoMatchingTransfer)
    );
}
