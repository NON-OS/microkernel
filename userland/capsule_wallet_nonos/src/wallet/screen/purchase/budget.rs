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

//! "Set budget" on an open voucher channel (v1.4.43): the global budget,
//! zero or past its end, set to the deposit for BUDGET_DAYS. SET_BUDGET
//! replaces every cap, so it carries this listing's cap with it.

use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;
use nonos_payment_proto::SetBudget;

use crate::wallet::meter::call::set_budget;
use crate::wallet::order::read::Named;
use crate::wallet::order::voucher_cap::{in_force, NO_CAP};
use crate::wallet::order::voucher_form::Budgeted;
use crate::wallet::state::State;

const BUDGET_DAYS: u64 = 30;

/// "Set budget": the deposit for BUDGET_DAYS, with this listing's cap.
pub fn set_budget_for_channel(state: &mut State) -> EventOutcome {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return EventOutcome::Idle };
    let (Budgeted::NoBudget, Ok(Named::Voucher(v))) = (p.form.budgeted, &p.read) else {
        return EventOutcome::Idle;
    };
    let now = crate::wallet::clock::now_s();
    if in_force(now) {
        p.form.budgeted = Budgeted::Set;
        return EventOutcome::Repaint;
    }
    let deposit = p.form.read(now).map(|(d, _, _)| d).unwrap_or(p.form.cap_wei);
    let listings = Vec::from([(v.listing_id, p.form.cap_wei)]);
    let body = SetBudget { cap: deposit, expiry: now + BUDGET_DAYS * 86_400, listings }.encode();
    p.form.budgeted = match body.map(|b| set_budget(&b)) {
        Some(Ok(())) => Budgeted::Set,
        _ => Budgeted::Failed(NO_CAP),
    };
    EventOutcome::Repaint
}
