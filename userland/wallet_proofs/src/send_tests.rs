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

//! A payment's gas limit and fee: a plain transfer at exactly its cost, a
//! call with its margin and never wrapping, and no signing on a missing,
//! zero or absurd fee.

use crate::wallet::send::gas::{fee_refusal, gas_limit, FEE_CEILING_WEI, PLAIN_GAS};

#[test]
fn a_plain_transfer_is_signed_at_exactly_its_cost() {
    assert_eq!(gas_limit(21_000, true), PLAIN_GAS);
    assert_eq!(gas_limit(20_000, true), PLAIN_GAS, "an estimate under the cost is not trusted");
}

#[test]
fn a_call_carries_its_margin() {
    assert_eq!(gas_limit(50_000, false), 60_000);
    assert_eq!(gas_limit(21_000, false), 25_200, "token data is a call whatever its estimate");
    /* A plain transfer the node says costs more, to a contract, gets the margin. */
    assert_eq!(gas_limit(30_000, true), 36_000);
}

#[test]
fn a_huge_estimate_saturates_instead_of_wrapping() {
    assert_eq!(gas_limit(u64::MAX, false), u64::MAX);
    assert!(gas_limit(u64::MAX / 2, false) >= u64::MAX / 2);
}

#[test]
fn no_payment_is_signed_on_a_fee_that_is_missing_zero_or_absurd() {
    assert!(fee_refusal(None).is_some());
    assert!(fee_refusal(Some(0)).is_some());
    assert!(fee_refusal(Some(FEE_CEILING_WEI + 1)).is_some());
    assert_eq!(fee_refusal(Some(FEE_CEILING_WEI)), None);
    assert_eq!(fee_refusal(Some(3_000_000_000)), None);
}
