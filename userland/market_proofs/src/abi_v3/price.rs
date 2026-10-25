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

//! The shown price against the commerce block the meter charges (B6).

use nonos_marketplace_abi::{price_verdict, MarketplaceEntry, PriceKind, PriceModel};
use nonos_marketplace_abi::{PriceVerdict, SettleMode};
use nonos_nox_config::{UNIT_INSTALL, UNIT_PROOF};

use super::fixtures::sold;

const PRICE: u128 = 4_000_000_000_000_000_000;

fn entry(
    mode: SettleMode,
    unit: [u8; 32],
    kind: PriceKind,
    amount: u128,
    period: u64,
) -> MarketplaceEntry {
    let mut e = sold().entries[0].clone();
    let c = e.commerce.as_mut().unwrap();
    (c.settle_mode, c.unit) = (mode, unit);
    c.unit_price = if mode == SettleMode::Free { 0 } else { PRICE };
    e.price = PriceModel { kind, amount_atomic: amount, period_seconds: period };
    e
}

#[test]
fn each_commerce_mode_has_one_honest_price() {
    use PriceKind::*;
    use SettleMode as M;
    let agree = |e: &MarketplaceEntry| price_verdict(e) == PriceVerdict::Agrees;
    assert!(agree(&entry(M::Free, UNIT_INSTALL, Free, 0, 0)));
    assert!(agree(&entry(M::Voucher, UNIT_INSTALL, OneTime, PRICE, 0)));
    assert!(agree(&entry(M::Voucher, UNIT_PROOF, UsageMetered, PRICE, 3_600)));
    assert!(agree(&entry(M::Escrow, UNIT_INSTALL, OneTime, PRICE, 0)));
    for (mode, unit, kind, amount, period, why) in [
        (M::Free, UNIT_INSTALL, OneTime, PRICE, 0, "free, shown as sold"),
        (M::Free, UNIT_INSTALL, Free, 1, 0, "free, with an amount"),
        (M::Voucher, UNIT_INSTALL, Free, 0, 0, "sold, shown as free"),
        (M::Voucher, UNIT_INSTALL, OneTime, PRICE - 1, 0, "a lower price shown"),
        (M::Voucher, UNIT_INSTALL, UsageMetered, PRICE, 0, "an install shown as metered"),
        (M::Voucher, UNIT_PROOF, OneTime, PRICE, 0, "metered shown as once"),
        (M::Voucher, UNIT_INSTALL, OneTime, PRICE, 60, "once, with a period"),
        (M::Escrow, UNIT_INSTALL, Subscription, PRICE, 60, "a subscription"),
        (M::Escrow, UNIT_INSTALL, OneTime, PRICE + 1, 0, "a higher price shown"),
    ] {
        assert_eq!(
            price_verdict(&entry(mode, unit, kind, amount, period)),
            PriceVerdict::Contradicts,
            "{why}"
        );
    }
}

#[test]
fn a_schema_2_entry_has_no_commerce_to_agree_with() {
    let mut e = sold().entries[0].clone();
    e.commerce = None;
    assert_eq!(price_verdict(&e), PriceVerdict::NotV3);
}
