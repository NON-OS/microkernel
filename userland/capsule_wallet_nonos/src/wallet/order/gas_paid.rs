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
//! The mark a started Shield withdraw for a send's gas leaves (v1.4.55):
//! set when the withdraw to the refund address starts while the send waits
//! for gas, and cleared when that withdraw fails, so the gas is never paid
//! twice for one step.

use super::send_step::SendStep;
use crate::wallet::state::State;

const ETH: u8 = 0;

fn waiting_gas(state: &mut State) -> Option<(&mut Option<u64>, [u8; 20])> {
    let p = state.meter.purchase.as_deref_mut()?;
    let e = p.escrow.as_mut()?;
    matches!(e.step, SendStep::Gas { .. }).then_some((&mut e.gas_paid_s, p.refund_to))
}

/// A Shield withdraw of `asset` to `deposit` has started.
pub fn started(state: &mut State, asset: u8, deposit: Option<[u8; 20]>) {
    let now = nonos_tls::rtc_now();
    if let Some((paid, refund_to)) = waiting_gas(state) {
        if asset == ETH && deposit == Some(refund_to) && refund_to != [0; 20] {
            *paid = Some(now);
        }
    }
}

/// The Shield withdraw failed: the gas may be paid again.
pub fn failed(state: &mut State) {
    if let Some((paid, _)) = waiting_gas(state) {
        *paid = None;
    }
}
