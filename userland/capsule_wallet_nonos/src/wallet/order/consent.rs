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
 * A purchase on its consent screen, one step a tick (B8): the market read
 * one call at a time, the screen opened only when it is all read
 * (guard/open.rs), then the shield's quote for an escrow price, so the
 * window never waits on more than one reply. A voucher listing's open
 * channel is looked for before its form (v1.4.48).
 */

use super::fees::{ask, settled};
use super::held::{after_page, Held};
use super::intake::{Pending, Step};
use super::read::Named;
use super::reading::{advance, Read};
use crate::wallet::meter::call::account;
use crate::wallet::state::State;

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return false };
    if let Some(at) = p.reading.take() {
        match advance(&p.names, at) {
            Read::More(next) => p.reading = Some(next),
            Read::Done(done) => super::guard::open::landed(state, done),
        }
        return true;
    }
    held(p) || settled(state) || ask(state)
}

/// A voucher listing's open channel looked for, one ACCOUNT page a tick.
fn held(p: &mut Pending) -> bool {
    let (Step::Consent, Ok(Named::Voucher(v)), Held::Asking(from)) = (&p.step, &p.read, p.held)
    else {
        return false;
    };
    p.held = after_page(account(from).as_ref(), from, v, crate::wallet::clock::now_s());
    true
}
