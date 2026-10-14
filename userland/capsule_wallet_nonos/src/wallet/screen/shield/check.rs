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
 * What each Shield action needs before it can be reviewed: an open shield
 * with no job running, a nox1 address of the right length and alphabet, a
 * positive amount with at most 18 decimals, and either matured notes or the
 * typed override. A grey button says which of these it waits for.
 */

pub use super::check_why::held_back;
use super::consts::OVERRIDE;
pub use super::typed::{address_ok, amount_ok};
use crate::wallet::state::shield_ui::ShieldUi;

/* Matured, or not yet counted (the service checks again), or overridden. */
pub fn wait_ok(ui: &ShieldUi) -> bool {
    ui.wait_left.is_none() || super::meter::waited(ui) || ui.override_text == OVERRIDE
}

pub fn deposit_ready(ui: &ShieldUi) -> bool {
    ui.opened && ui.waiting.is_none() && ui.size.is_some()
}

pub fn withdraw_ready(ui: &ShieldUi) -> bool {
    ui.opened && ui.waiting.is_none() && ui.size.is_some() && wait_ok(ui)
}

pub fn send_ready(ui: &ShieldUi) -> bool {
    ui.opened && ui.waiting.is_none() && address_ok(&ui.to) && amount_ok(&ui.amount) && wait_ok(ui)
}
