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

/*
 * A voucher channel held to the person's cap (B8, v1.4.43): once it is
 * open, LISTING_CAP. A global budget of zero or past its end is offered as
 * the deposit for 30 days, never over one in force (screen/purchase/
 * budget.rs).
 */

use super::intake::Step;
use super::read::Named;
use super::voucher_form::Budgeted;
use crate::wallet::meter::call::account;
use crate::wallet::meter::order_calls::listing_cap;
use crate::wallet::state::State;

pub const NO_CAP: &str = "The meter did not take the listing's cap; set it from the Meter screen.";

pub fn in_force(now: u64) -> bool {
    account().is_some_and(|a| a.cap > 0 && a.expiry > now)
}

/// Once the channel is open, its cap once. Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return false };
    let (Step::Opened(_), Ok(Named::Voucher(v))) = (&p.step, &p.read) else { return false };
    if p.form.budgeted != Budgeted::Unasked {
        return false;
    }
    let now = nonos_tls::rtc_now();
    p.form.budgeted = match listing_cap(v.listing_id, p.form.cap_wei) {
        Err(_) => Budgeted::Failed(NO_CAP),
        Ok(()) if in_force(now) => Budgeted::Set,
        Ok(()) => Budgeted::NoBudget,
    };
    true
}
