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

//! The issuance decision.

use nonos_nox_license::{price_of, Entitlement};
use nonos_nox_receipt::verify_payment;

use super::types::{IssueError, Issued, PaidTo, Terms};
use crate::spent::SpentSet;

/// Decide whether the receipt earns an entitlement for `tool_id`.
///
/// On success the funding hash is recorded in `spent` and the returned grant is
/// ready to sign, on `terms`. The number of uses is what arrived divided by
/// the unit price, so paying a multiple of the price buys that many runs at
/// once.
pub fn issue(
    spent: &mut SpentSet,
    tool_id: u32,
    to: &PaidTo,
    tx_hash: [u8; 32],
    receipt_json: &[u8],
    terms: &Terms,
) -> Result<Issued, IssueError> {
    let Terms { now, ttl_secs, nonce, device } = *terms;
    let price = price_of(tool_id).ok_or(IssueError::UnknownTool)?;
    if to.payee == [0u8; 20] || to.token == [0u8; 20] {
        return Err(IssueError::NoPayee);
    }

    let payment =
        verify_payment(receipt_json, &to.payee, price, &to.token).map_err(IssueError::Payment)?;

    // Recorded only once everything has passed, so a failure never burns it.
    if spent.contains(&tx_hash) {
        return Err(IssueError::Replay);
    }

    // Price is non-zero and the payment cleared it, so this is at least one.
    let uses = (payment.amount / price).min(u32::MAX as u128) as u32;
    let expiry = if ttl_secs == 0 { 0 } else { now.saturating_add(ttl_secs) };
    let entitlement = Entitlement {
        tool_id,
        buyer: payment.from,
        device,
        uses,
        issued_at: now,
        expiry,
        tx_hash,
        nonce,
    };

    let spent_set_full = spent.record(tx_hash);
    Ok(Issued { entitlement, spent_set_full })
}
