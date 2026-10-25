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

//! Nothing can be bought or quoted while a named purchase's terms are read
//! (B8, v1.4.36): consent waits for every answer and read_named.

use nonos_market_proto::Commerce;
use nonos_marketplace_abi::SettleMode;

use super::deposit_fixture::K;
use super::fixture::buy;
use super::gate_fixture::names;
use super::ids_fixture::commerce_for;
use super::market_answers::commerce_body;
use super::market_drive::escrow_market;
use crate::order::buy_gate::{buyable, to_quote};
use crate::order::fees_kind::Fees;
use crate::order::pending_step::Step;
use crate::order::read::Named;
use crate::order::reading_step::{Read, Reading};
use crate::order::terms::answer::answered;

/// Whether Buy or the quote would act on `r` at consent.
fn consents(r: &Result<Named, &'static str>) -> bool {
    let quoted = Fees::Quoted("0.01".into());
    let buys = buyable(&Step::Consent, r, &quoted).is_some();
    buys || to_quote(&Step::Consent, r, &Fees::Unasked).is_some()
}

#[test]
fn no_consent_before_all_three_are_read_and_the_terms_pass() {
    let untrusted = Commerce { binding_verdict: 1, ..commerce_for(&names(), SettleMode::Escrow) };
    let mut refused = escrow_market(buy().amount);
    refused[0] = commerce_body(&untrusted);
    for (m, passes) in [(escrow_market(buy().amount), true), (refused, false)] {
        let (mut at, mut read) = (Some(Reading::Commerce), Err("Reading the listing's terms."));
        for answer in &m {
            assert!(!consents(&read), "nothing to consent to while reading");
            let Some(now) = at.take() else { break };
            match answered(&K, &names(), now, answer).unwrap() {
                Read::More(next) => at = Some(next),
                Read::Done(done) => read = done.map(|t| t.named),
            }
        }
        assert!(at.is_none(), "the last answer due ends the reading");
        assert_eq!(consents(&read), passes);
    }
}
