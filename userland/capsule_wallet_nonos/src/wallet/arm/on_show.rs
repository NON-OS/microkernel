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

//! Which guarded screen shows (moves.rs), read from what the paint draws:
//! none while the lock or the backup words are up, or before an account.

use super::moves::Shown;
use crate::wallet::intent::IntentStep;
use crate::wallet::order::escrow_offer::Offer;
use crate::wallet::order::guard::poll::guarding;
use crate::wallet::order::intake::Step;
use crate::wallet::order::voucher_form::Budgeted;
use crate::wallet::screen::purchase::escrow_words::offered;
use crate::wallet::send::STAGE_REVIEW;
use crate::wallet::state::{State, VIEW_EXPORT, VIEW_METER, VIEW_NOX, VIEW_PUBLISHER};
use crate::wallet::state::{VIEW_PURCHASE, VIEW_SEND, VIEW_SHIELD};

pub fn on_show(state: &State) -> Option<Shown> {
    if state.locked || state.keys.backup_active {
        return None;
    }
    match (state.view, state.acct.address_ready) {
        (VIEW_EXPORT, _) => (!state.keys.export_active).then_some(Shown::Export),
        (_, false) => None,
        (VIEW_PURCHASE, _) => purchase(state),
        (VIEW_SEND, _) => (state.send.stage == STAGE_REVIEW).then_some(Shown::PayReview),
        (VIEW_NOX, _) => state.stake.draft.is_some().then_some(Shown::StakeReview),
        (VIEW_SHIELD, _) => super::on_shield::shield(state),
        (VIEW_METER, _) => Some(Shown::Meter),
        (VIEW_PUBLISHER, _) => publisher(state),
        _ => None,
    }
}

/// As the purchase screen's footer reads (screen/purchase/click.rs).
fn purchase(state: &State) -> Option<Shown> {
    if guarding(state) {
        return Some(Shown::Consent);
    }
    let p = state.meter.purchase.as_deref()?;
    match (offered(p), &p.step) {
        (Some(Offer::Dispute | Offer::Refund | Offer::Again), _) => Some(Shown::Escrow),
        (Some(_), _) | (None, Step::Fund(_)) => None,
        _ => (p.form.budgeted == Budgeted::NoBudget).then_some(Shown::Budget),
    }
}

/// A transaction's review only once its rows are there to be drawn whole;
/// nothing guarded while it is quoted or sent; else the entry's signing.
fn publisher(state: &State) -> Option<Shown> {
    match &state.publisher.sending {
        Some(s) => matches!(s.step, IntentStep::Review { .. }).then_some(Shown::PublisherReview),
        None if state.publisher.attest_ask.is_some() => Some(Shown::AttestReview),
        None => state.publisher.entry.is_some().then_some(Shown::Publish),
    }
}
