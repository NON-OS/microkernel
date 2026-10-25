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

//! The Shield's fee is a NOX amount or there is no Buy, only a refusal
//! said in a sentence; a price it will not fund is refused saying why.

use super::consent_terms::WALLET;
use super::deposit_fixture::K;
use super::gate_fixture::names;
use super::record_fixture::{read, recorded};
use crate::order::buy_gate::buyable;
use crate::order::fees_kind::{fee_wei, quote_answer, Fees, FEE_UNREAD};
use crate::order::pending_step::Step;
use crate::order::read::Named;
use crate::order::terms::lines::lines;
use crate::order::terms::words::REFUSED;

const UNREADABLE: [&str; 8] = ["", "not reported", "1.5 NOX", "12 gwei", ".5", "5.", "-1", "1e3"];
const ALSO: [&str; 3] = [" 1", "0x10", "0.0000000000000000001"];

#[test]
fn a_fee_is_read_as_nox_exactly_or_not_at_all() {
    let one = 1_000_000_000_000_000_000;
    assert_eq!(fee_wei("0"), Some(0));
    assert_eq!(fee_wei("1"), Some(one));
    assert_eq!(fee_wei("0.0125"), Some(12_500_000_000_000_000));
    assert_eq!(fee_wei("0.000000000000000001"), Some(1));
    assert_eq!(fee_wei("340282366920938463463"), Some(340282366920938463463 * one));
    assert_eq!(fee_wei("340282366920938463464"), None, "past u128 as wei");
    UNREADABLE.iter().chain(&ALSO).for_each(|f| assert_eq!(fee_wei(f), None, "{f:?}"));
}

#[test]
fn a_fee_unreadable_as_nox_gives_no_buy_and_the_refusal_sentence() {
    let n = names();
    let (b, r) = read(&n, &recorded(&n, b"Quiet Notes"));
    let read = Ok(Named::Escrow(b.clone()));
    for fee in UNREADABLE.iter().map(|f| Some(*f)).chain([None]) {
        let fees = quote_answer(None, fee);
        assert!(matches!(&fees, Fees::Refused(why) if why == FEE_UNREAD), "{fee:?}");
        assert!(buyable(&Step::Consent, &read, &fees).is_none());
        let quoted_raw = Fees::Quoted(fee.unwrap_or("").into());
        assert!(buyable(&Step::Consent, &read, &quoted_raw).is_none(), "Buy reads it again");
        for f in [fees, quoted_raw] {
            let l = lines(&K, WALLET, &n, &r, &b, &f);
            assert_eq!((l.lead.as_str(), l.action), (FEE_UNREAD, None));
            assert!(l.rows.iter().all(|(k, _)| *k != "fee"), "no fee without its unit");
        }
    }
    assert!(matches!(quote_answer(None, Some("0.0125")), Fees::Quoted(f) if f == "0.0125"));
}

#[test]
fn a_price_the_shield_will_not_fund_is_refused_saying_why() {
    let n = names();
    let (b, r) = read(&n, &recorded(&n, b"Quiet Notes"));
    let fees = quote_answer(Some("not a standard amount today"), Some("0.0125"));
    let l = lines(&K, WALLET, &n, &r, &b, &fees);
    assert_eq!(REFUSED, "This price cannot be paid privately yet, so the wallet will not pay it.");
    assert_eq!((l.lead.as_str(), l.action), (REFUSED, None));
    assert!(l.rows.contains(&("shield", String::from("not a standard amount today"))));
}
