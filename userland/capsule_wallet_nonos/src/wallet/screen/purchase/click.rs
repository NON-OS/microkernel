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

//! Buy starts the order; Close drops the purchase, never while a job runs.

use nonos_app_skeleton::EventOutcome;
use nonos_payment_proto::{EINVAL, OP_ORDER_OFFER};

use super::fund::funding;
use super::words::closable;
use crate::wallet::meter::call::start;
use crate::wallet::order::fees::Fees;
use crate::wallet::order::fresh::fresh_address;
use crate::wallet::order::intake::Step;
use crate::wallet::order::read::Named;
use crate::wallet::order::said::{failed, Stage};
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME};

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Dismiss => state.failure = None,
        Press::Footer(1) if funding(state) => return super::fund::fund(state),
        Press::Footer(1) => return confirm(state),
        Press::Footer(0) if state.meter.purchase.as_deref().is_some_and(closable) => {
            state.meter.purchase = None;
            state.view = VIEW_HOME;
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

/// The person confirmed: draw the refund address and start the offer.
fn confirm(state: &mut State) -> EventOutcome {
    let Some(p) = state.meter.purchase.as_mut() else { return EventOutcome::Idle };
    let (Step::Consent, Ok(Named::Escrow(buy))) = (&p.step, &p.read) else {
        return EventOutcome::Idle;
    };
    if !matches!(p.fees, Fees::Quoted(_)) {
        return EventOutcome::Idle;
    }
    p.step = match fresh_address() {
        Err(why) => Step::Failed(why),
        Ok(to) => {
            p.refund_to = to;
            let body = buy.offer_ask().encode().ok_or(EINVAL);
            match body.and_then(|b| start(OP_ORDER_OFFER, &b)) {
                Ok(id) => Step::Offering(id),
                Err(s) => Step::Failed(failed(Stage::Offer, s)),
            }
        }
    };
    EventOutcome::Repaint
}
