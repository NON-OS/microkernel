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
 * A voucher listing's channel opened from the purchase screen (B8,
 * v1.4.38): OPEN START as a meter job (approve, then open, each signed by
 * the meter); opened.rs follows it until the opening is mined.
 * close() pays the unspent deposit to reclaimTo, a fresh Shield address.
 */

use alloc::boxed::Box;

use nonos_payment_proto::{EBUSY, EGAS, ENOPERSIST, OP_OPEN};

use super::fresh::fresh_address;
use super::intake::Step;
use super::read::Named;
use crate::wallet::meter::call::start;
use crate::wallet::state::State;

pub(super) fn refused(status: i32) -> &'static str {
    match status {
        EGAS => "The meter's ETH is short of gas, or the fees are above its caps.",
        EBUSY => "The meter is busy with another job; try again soon.",
        ENOPERSIST => "The meter has no disk to keep the channel on.",
        _ => "The meter could not open the channel.",
    }
}

/// Open channel: the form as typed, a fresh reclaim address, OPEN START.
pub fn open(state: &mut State) -> Option<&'static str> {
    let p = state.meter.purchase.as_deref_mut()?;
    let (Step::Consent, Ok(Named::Voucher(v))) = (&p.step, &p.read) else { return None };
    let (amount, expiry) = match p.form.read(nonos_tls::rtc_now()) {
        Ok(read) => read,
        Err(why) => return Some(why),
    };
    let s = match fresh_address().and_then(|to| v.open(amount, expiry, to, nonos_tls::rtc_now())) {
        Ok(s) => s,
        Err(why) => return Some(why),
    };
    p.step = match start(OP_OPEN, &s.encode()) {
        Ok(id) => Step::Opening(id, Box::new(s)),
        Err(e) => Step::Failed(refused(e)),
    };
    None
}
