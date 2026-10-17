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

//! Fund's withdraw (B8, the plan): set up only at the funding step on quoted
//! escrow terms, for the block's price written exactly, in NOX, to the
//! deposit address the meter computed.

use super::fixture::{buy, made, offer, NOW, REFUND_PAIR};
use super::gate_fixture::{fees, reads, steps};
use crate::order::fees_kind::{exact, Fees};
use crate::order::fund_ask::{fund_ask, FundAsk};
use crate::order::pending_step::Step;
use crate::order::read::Named;
use crate::wallet::state::shield_ui::ASSET_NOX;

#[test]
fn fund_is_set_up_only_at_the_funding_step_on_quoted_escrow_terms() {
    let mut set_up = 0;
    for (i, step) in steps().iter().enumerate() {
        for read in &reads() {
            for (f, fee) in fees().iter().enumerate() {
                let got = fund_ask(step, read, fee);
                let escrow = matches!(read, Ok(Named::Escrow(_)));
                if matches!(step, Step::Fund(_)) && escrow && matches!(fee, Fees::Quoted(_)) {
                    assert!(got.is_some());
                    set_up += 1;
                } else {
                    assert_eq!(got, None, "step {i}, read {read:?}, fees {f}");
                }
            }
        }
    }
    assert_eq!(set_up, 1);
}

#[test]
fn the_withdraw_is_the_exact_price_in_nox_to_the_meters_deposit() {
    let quoted = Fees::Quoted("0.02".into());
    for (amount, deposit) in [(buy().amount, [0xDD; 20]), (1_234_500_000_000_000_001, [0x0E; 20])] {
        let b = crate::order::buy::Buy { amount, ..buy() };
        let mut m = made(&b.order_new(&offer(), REFUND_PAIR, NOW));
        m.deposit = deposit;
        let got = fund_ask(&Step::Fund(Box::new(m)), &Ok(Named::Escrow(b)), &quoted);
        let want = FundAsk { asset: ASSET_NOX, amount: exact(amount), to: deposit };
        assert_eq!(got, Some(want));
    }
    assert_eq!(exact(1_234_500_000_000_000_001), "1.234500000000000001", "never rounded");
    assert_eq!(ASSET_NOX, 1, "the Shield's NOX, not ETH (0)");
}
