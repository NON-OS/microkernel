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

//! A voucher listing's open channel on the purchase screen (v1.4.48): Use
//! it keeps that channel; Open another offers the form for a second one.

use nonos_payment_proto::AccountRow;

use crate::wallet::order::held::Held;
use crate::wallet::order::intake::Step;
use crate::wallet::state::State;

/// The open channel a voucher listing already has, while it is offered.
pub fn held(state: &State) -> Option<AccountRow> {
    let p = state.meter.purchase.as_deref()?;
    match (&p.step, p.held) {
        (Step::Consent, Held::Open(r)) => Some(r),
        _ => None,
    }
}

/// Footer 1 uses the open channel; footer 2 opens another.
pub fn choose(state: &mut State, footer: u8) {
    let id = held(state).map_or([0; 32], |r| r.channel_id);
    let Some(p) = state.meter.purchase.as_deref_mut() else { return };
    match footer {
        1 => p.step = Step::Kept(id),
        _ => p.held = Held::Another,
    }
}
