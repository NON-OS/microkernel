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

//! The store's price line (B9): exact NOX, the unit and how it settles, and
//! nothing priced until both the publisher and the payee stand behind it.

use crate::store::price::{gas_line, nox_amount, price_line, Price, ESCROW, FREE, VOUCHER};

fn price(mode: u8, unit_price: u128, trusted: bool) -> Price {
    Price { mode, unit: b"install", unit_price, payee: [0; 20], trusted }
}

#[test]
fn a_price_reads_exact_nox_per_unit_and_how_it_settles() {
    let four = 4_000_000_000_000_000_000;
    assert_eq!(price_line(&price(VOUCHER, four, true)), b"4 NOX per install, paid by voucher");
    assert_eq!(
        price_line(&price(ESCROW, four / 8, true)),
        b"0.5 NOX per install, bought through escrow"
    );
    assert_eq!(price_line(&price(FREE, four, false)), b"Free");
    assert_eq!(price_line(&price(VOUCHER, four, false)), b"Price not confirmed by its payee");
}

#[test]
fn nox_is_never_rounded() {
    assert_eq!(nox_amount(0), b"0");
    assert_eq!(nox_amount(1), b"0.000000000000000001");
    assert_eq!(nox_amount(1_234_500_000_000_000_000), b"1.2345");
    assert_eq!(nox_amount(u128::MAX), b"340282366920938463463.374607431768211455");
}

#[test]
fn only_a_trusted_voucher_price_says_the_meter_pays_the_settles_gas() {
    let gas: &[u8] = b"Each install also spends one settle's gas from the meter's ETH";
    assert_eq!(gas_line(&price(VOUCHER, 1, true)), Some(gas));
    assert!(gas.len() < 80, "one short line");
    for (mode, trusted) in [(VOUCHER, false), (ESCROW, true), (FREE, true)] {
        assert_eq!(gas_line(&price(mode, 1, trusted)), None);
    }
}
