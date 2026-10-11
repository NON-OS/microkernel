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

use nonos_marketplace_abi::{Commerce, SettleMode};
use nonos_nox_meter::pay::priced;
use nonos_nox_meter::MeterError;

use super::fixture::{commerce, offer, request, NOX};

#[test]
fn only_voucher_listings_at_or_above_their_minimum_are_metered() {
    let r = request();
    let escrow = offer(&Commerce { settle_mode: SettleMode::Escrow, ..commerce() });
    assert_eq!(priced(&r, &escrow, &NOX), Err(MeterError::NotMetered));
    let dear = offer(&Commerce { min_charge: 36, ..commerce() });
    assert_eq!(priced(&r, &dear, &NOX), Err(MeterError::BelowMinimum));
    let free = offer(&Commerce { unit_price: 0, min_charge: 0, ..commerce() });
    let mut zero = r;
    zero.unit_price = 0;
    assert_eq!(priced(&zero, &free, &NOX), Err(MeterError::BelowMinimum));
}
