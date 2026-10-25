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

//! The payment decision: does this receipt pay the payee?

use super::transfer::transfer_to;
use super::types::{Payment, ReceiptError};
use crate::logs::{for_each_log, status_ok};

/// Accept the receipt only if it shows a successful transaction with a
/// `Transfer` of `token` to `payee` of at least `min_amount`, and report who
/// paid and how much arrived.
///
/// Neither address is this crate's to know. The payee comes from the signed
/// listing and the token from `nonos_nox_config`, so a redeploy or a new
/// publisher never means a new build of the verifier.
pub fn verify_payment(
    receipt_json: &[u8],
    payee: &[u8; 20],
    min_amount: u128,
    token: &[u8; 20],
) -> Result<Payment, ReceiptError> {
    match status_ok(receipt_json) {
        Some(true) => {}
        Some(false) => return Err(ReceiptError::Reverted),
        None => return Err(ReceiptError::NoStatus),
    }

    let mut found: Option<Payment> = None;
    for_each_log(receipt_json, |log| match transfer_to(log, payee, min_amount, token) {
        Some(p) => {
            found = Some(p);
            true
        }
        None => false,
    });

    found.ok_or(ReceiptError::NoMatchingTransfer)
}
