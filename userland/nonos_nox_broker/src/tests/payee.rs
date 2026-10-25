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

use nonos_nox_receipt::ReceiptError;

use super::fixtures::{paying, price, to};
use super::refusals::try_issue;
use crate::issue::{IssueError, PaidTo};
use crate::spent::SpentSet;

#[test]
fn a_payment_to_another_payee_is_refused() {
    let mut spent = SpentSet::new();
    let elsewhere = PaidTo { payee: [0x5b; 20], ..to() };
    let out = try_issue(&mut spent, &elsewhere, [8; 32], &paying(price()));
    assert_eq!(out, Err(IssueError::Payment(ReceiptError::NoMatchingTransfer)));
}

#[test]
fn a_zero_payee_or_token_sells_nothing() {
    let mut spent = SpentSet::new();
    let r = paying(price());
    for bad in [PaidTo { payee: [0; 20], ..to() }, PaidTo { token: [0; 20], ..to() }] {
        assert_eq!(try_issue(&mut spent, &bad, [9; 32], &r), Err(IssueError::NoPayee));
    }
    assert!(spent.is_empty());
}
