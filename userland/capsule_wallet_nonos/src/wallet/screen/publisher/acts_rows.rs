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

//! The Bond's rows and each release's, under the attests: each only asks
//! settle for a quote, and its review signs. Withdraw the bond waits on
//! bondOf (Q6), the credit on credit (Q7), the bond on MIN_BOND and the
//! allowance: greyed until each is read, none guessed.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_marketplace_abi::MarketplaceEntry;

use crate::wallet::arm::rows::{ROW_INTENT, ROW_RELEASE};
use crate::wallet::intent::{credit_of, may_withdraw};
use crate::wallet::send::amount_text;
use crate::wallet::state::State;

/// Each row's text, whether it is lit, and its row number.
pub fn rows(state: &State, e: &MarketplaceEntry) -> Vec<(String, bool, u8)> {
    let (p, me) = (&state.publisher, &state.acct.address);
    let v = &p.views;
    let n = Some(amount_text(&p.amount)).filter(|t| !t.is_empty()).unwrap_or("0".into());
    let bond = !p.amount.is_zero() && v.min_bond.is_some() && v.allowance.is_some();
    let unlocked =
        v.bond.as_deref().is_some_and(|b| may_withdraw(b, crate::wallet::clock::now_s()));
    let credit = v.credit.is_some_and(|c| credit_of(me, &c).is_ok());
    let mut out = Vec::from([
        (format!("Bond {n} NOX for this publisher key"), bond, ROW_INTENT),
        ("Name the delivery agent".into(), !p.agent.is_empty(), ROW_INTENT + 1),
        ("Start unbonding".into(), true, ROW_INTENT + 2),
        ("Withdraw the bond".into(), unlocked, ROW_INTENT + 3),
        ("Withdraw the Bond's credit".into(), credit, ROW_INTENT + 4),
    ]);
    for (i, rel) in (0u8..).zip(e.releases.iter().take(16)) {
        let (r, id) = (ROW_RELEASE + 3 * i, &rel.release_id);
        let anchors = rel.ext.as_ref().map_or(0, |x| x.proof_anchors.len());
        out.push((format!("Publish {id}"), !p.uri.is_empty(), r));
        out.push((format!("Withdraw {id}"), true, r + 1));
        out.push((format!("Anchor {anchors} proofs of {id}"), anchors > 0, r + 2));
    }
    out
}
