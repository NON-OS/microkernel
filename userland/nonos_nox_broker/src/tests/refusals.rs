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

use nonos_nox_license::ToolId;
use nonos_nox_receipt::ReceiptError;

use super::fixtures::{paying, price, receipt, terms, to};
use crate::issue::{issue, IssueError, PaidTo};
use crate::spent::SpentSet;

const RECON: u32 = ToolId::Recon.id();

pub(super) fn try_issue(
    spent: &mut SpentSet,
    to: &PaidTo,
    tx: [u8; 32],
    r: &str,
) -> Result<(), IssueError> {
    issue(spent, RECON, to, tx, r.as_bytes(), &terms()).map(|_| ())
}

#[test]
fn the_same_transaction_cannot_be_redeemed_twice() {
    let mut spent = SpentSet::new();
    let r = paying(price());
    assert!(try_issue(&mut spent, &to(), [0xCD; 32], &r).is_ok());
    assert_eq!(try_issue(&mut spent, &to(), [0xCD; 32], &r), Err(IssueError::Replay));
}

#[test]
fn an_unknown_tool_is_refused_before_any_payment_check() {
    let mut spent = SpentSet::new();
    let r = paying(price());
    let out = issue(&mut spent, 999, &to(), [3; 32], r.as_bytes(), &terms());
    assert_eq!(out.map(|_| ()), Err(IssueError::UnknownTool));
    assert!(spent.is_empty());
}

#[test]
fn an_underpaying_receipt_is_refused_and_records_nothing() {
    let mut spent = SpentSet::new();
    let r = paying(price() - 1);
    let out = try_issue(&mut spent, &to(), [4; 32], &r);
    assert_eq!(out, Err(IssueError::Payment(ReceiptError::NoMatchingTransfer)));
    // A failed attempt must not burn the hash: the buyer can retry with a real
    // payment under the same tx once it confirms.
    assert!(!spent.contains(&[4; 32]));
}

#[test]
fn a_reverted_payment_is_refused() {
    let mut spent = SpentSet::new();
    let out = try_issue(&mut spent, &to(), [5; 32], &receipt("0x0", price()));
    assert_eq!(out, Err(IssueError::Payment(ReceiptError::Reverted)));
}
