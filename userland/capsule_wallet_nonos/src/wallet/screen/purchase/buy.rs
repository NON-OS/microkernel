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

//! Buy on an escrow purchase: once the Shield has quoted the price, the
//! refund address is drawn and ORDER_OFFER starts in the meter.

use nonos_app_skeleton::EventOutcome;
use nonos_payment_proto::{EINVAL, OP_ORDER_OFFER};

use crate::wallet::meter::call::start;
use crate::wallet::order::fees::Fees;
use crate::wallet::order::fresh::fresh_address;
use crate::wallet::order::intake::Step;
use crate::wallet::order::read::Named;
use crate::wallet::order::said::{failed, Stage};
use crate::wallet::state::State;

/// The person confirmed: draw the refund address and start the offer.
pub fn confirm(state: &mut State) -> EventOutcome {
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
