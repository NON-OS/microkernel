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
//! The mark a started Shield withdraw for gas leaves (v1.4.55, v1.4.56):
//! set when an ETH withdraw starts to the address a send waits on for gas,
//! the refund address of a dispute or refund, or a fresh address whose NOX
//! is being moved into the Shield; cleared when the withdraw to that
//! address fails, so the gas is never paid twice for one step.

use super::send_resume::pays_gas;
use super::send_step::SendStep;
use crate::wallet::meter::MeterUi;
use crate::wallet::state::State;

/// Each mark a gas withdraw may set now, with the address it pays.
fn marks(meter: &mut MeterUi) -> impl Iterator<Item = (&mut Option<u64>, [u8; 20])> {
    let escrow = meter.purchase.as_deref_mut().and_then(|p| {
        let e = p.escrow.as_mut()?;
        matches!(e.step, SendStep::Gas { .. }).then_some((&mut e.gas_paid_s, p.refund_to))
    });
    let lacks = |m: &&mut crate::wallet::shield::fresh_move::Moving| {
        m.review.as_ref().is_some_and(|r| r.short_eth > 0)
    };
    let moving = meter.landed.moving.as_mut().filter(lacks).map(|m| (&mut m.gas_paid_s, m.address));
    escrow.into_iter().chain(moving)
}

/// A Shield withdraw of `asset` to `deposit` has started.
pub fn started(state: &mut State, asset: u8, deposit: Option<[u8; 20]>) {
    let now = nonos_tls::rtc_now();
    for (paid, to) in marks(&mut state.meter) {
        if pays_gas(asset, deposit, to) {
            *paid = Some(now);
        }
    }
}

/// The Shield withdraw failed: the gas to the address it paid may go again.
pub fn failed(state: &mut State) {
    let deposit = state.shield_ui.to_deposit;
    for (paid, to) in marks(&mut state.meter) {
        if deposit == Some(to) {
            *paid = None;
        }
    }
}
