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
use crate::verify::{verify_payment, ReceiptError};

fn refused(r: &str) -> ReceiptError {
    verify_payment(r.as_bytes(), &PAYEE, PRICE, &token()).expect_err("refused")
}

#[test]
fn a_reverted_transaction_is_rejected() {
    let r = receipt("0x0", &token_addr(), &paid_topics(), &amount_word(PRICE));
    assert_eq!(refused(&r), ReceiptError::Reverted);
}

#[test]
fn a_receipt_without_status_is_rejected() {
    assert_eq!(refused("{\"result\":{\"logs\":[]}}"), ReceiptError::NoStatus);
}

#[test]
fn a_transfer_of_another_token_is_rejected() {
    let r = receipt("0x1", &hex_of(&[0x11u8; 20]), &paid_topics(), &amount_word(PRICE));
    assert_eq!(refused(&r), ReceiptError::NoMatchingTransfer);
}

#[test]
fn a_transfer_to_someone_else_is_rejected() {
    let topics = [transfer_topic(), addr_topic(&BUYER), addr_topic(&[0xDEu8; 20])];
    let r = receipt("0x1", &token_addr(), &topics, &amount_word(PRICE));
    assert_eq!(refused(&r), ReceiptError::NoMatchingTransfer);
}

#[test]
fn a_non_transfer_event_is_ignored() {
    // Approval(address,address,uint256) topic0, not Transfer.
    let approval = [
        0x8c, 0x5b, 0xe1, 0xe5, 0xeb, 0xec, 0x7d, 0x5b, 0xd1, 0x4f, 0x71, 0x42, 0x7d, 0x1e, 0x84,
        0xf3, 0xdd, 0x03, 0x14, 0xc0, 0xf7, 0xb2, 0x29, 0x1e, 0x5b, 0x20, 0x0a, 0xc8, 0xc7, 0xc3,
        0xb9, 0x25,
    ];
    let topics = [hex_of(&approval), addr_topic(&BUYER), addr_topic(&PAYEE)];
    let r = receipt("0x1", &token_addr(), &topics, &amount_word(PRICE));
    assert_eq!(refused(&r), ReceiptError::NoMatchingTransfer);
}

#[test]
fn a_truncated_receipt_fails_closed() {
    let r = receipt("0x1", &token_addr(), &paid_topics(), &amount_word(PRICE));
    let cut = &r.as_bytes()[..r.len() / 2];
    assert!(verify_payment(cut, &PAYEE, PRICE, &token()).is_err());
}
