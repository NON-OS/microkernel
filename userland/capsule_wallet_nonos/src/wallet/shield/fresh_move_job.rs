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
//! The wallet's side of SHIELD_FROM_FRESH (v1.4.56): "Move to the Shield"
//! on a landed row reviews moving its NOX; Confirm sends the approval, or
//! the deposits; the gas the address lacks comes from a Shield ETH
//! withdraw, held once started until it fails or GAS_WAIT_S pass.

use alloc::format;
use alloc::vec::Vec;

use nonos_app_skeleton::EventOutcome;
use shield_wire::{hex, OP_SHIELD_FROM_FRESH, SHIELD_FRESH_CONFIRM, SHIELD_FRESH_REVIEW};

use super::fresh_move::{reviewed, sent, Moving};
use super::gas_withdraw::withdraw_gas;
use crate::wallet::order::send_quote::size_for;
use crate::wallet::state::State;

const MOST_SENT: usize = 32;

/// Review moving `amount` wei of NOX, or of ETH, at `address` into the Shield.
pub fn start(state: &mut State, address: [u8; 20], amount: u128, eth: bool) {
    let coin = if eth { "ETH" } else { "NOX" };
    let fields = [SHIELD_FRESH_REVIEW, &hex(&address), &format!("{amount}"), coin];
    if super::job::start(state, OP_SHIELD_FROM_FRESH, &fields) {
        let l = &mut state.meter.landed;
        let kept = l.moving.take().filter(|m| m.address == address).and_then(|m| m.gas_paid_s);
        l.moving = Some(Moving { address, amount, eth, gas_paid_s: kept, ..Moving::default() });
    }
}

/// Confirm the review shown, if it can go as it is.
pub fn confirm(state: &mut State) {
    let r = state.meter.landed.moving.as_ref().and_then(|m| m.review.as_ref());
    let Some(id) = r.filter(|r| r.refusal.is_none() && r.short_eth == 0).map(|r| r.id) else {
        return;
    };
    super::job::start(state, OP_SHIELD_FROM_FRESH, &[SHIELD_FRESH_CONFIRM, &format!("{id}")]);
}

/// The job's answer: a review, or what a confirm sent.
pub fn applied(state: &mut State, body: &str) {
    let Some(m) = state.meter.landed.moving.as_mut() else { return };
    match reviewed(body) {
        Some(r) => (m.review, m.sent, m.stopped) = (Some(r), Vec::new(), None),
        None => (m.review, (m.sent, m.stopped)) = (None, sent(body, MOST_SENT)),
    }
}

/// The gas the address lacks, as a standard ETH withdraw from the Shield.
pub fn pay_gas(state: &mut State) -> EventOutcome {
    let m = state.meter.landed.moving.as_ref();
    let short = m.and_then(|m| m.review.as_ref().map(|r| (m.address, r.short_eth)));
    match short.filter(|s| s.1 > 0).and_then(|(to, lacks)| Some((to, size_for(lacks)?))) {
        Some((to, size)) => withdraw_gas(state, to, size),
        None => EventOutcome::Idle,
    }
}
