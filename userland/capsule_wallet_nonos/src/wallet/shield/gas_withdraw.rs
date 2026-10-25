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

//! The gas a send from a fresh address lacks, paid from the Shield
//! (v1.4.51, v1.4.56): a standard ETH withdraw to that address, reviewed
//! on the Shield's withdraw screen like any other.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::order::fees::exact;
use crate::wallet::shield::{actions, open};
use crate::wallet::state::shield_ui::SHIELD_WITHDRAW;
use crate::wallet::state::{State, VIEW_SHIELD};

const OFF_SHIELD: &str = "The Shield is on another network: switch to Sepolia to pay the gas.";
const ETH: u8 = 0;

/// The Shield's review of a standard ETH withdraw of `size` wei to `to`.
pub fn withdraw_gas(state: &mut State, to: [u8; 20], size: u128) -> EventOutcome {
    let amount = exact(size);
    if !open::here() {
        state.failure = Some(OFF_SHIELD);
        return EventOutcome::Repaint;
    }
    let ui = &mut state.shield_ui;
    (ui.screen, ui.asset, ui.size, ui.amount) = (SHIELD_WITHDRAW, ETH, None, amount);
    (ui.payee.to_meter, ui.payee.to_deposit, ui.failure) = (false, Some(to), None);
    state.view = VIEW_SHIELD;
    state.scroll = 0;
    actions::review_withdraw(state);
    EventOutcome::Repaint
}
