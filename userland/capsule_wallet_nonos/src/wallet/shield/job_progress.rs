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

//! What a running job reports before it ends: the open's address, the
//! prover's phase and fraction.

use alloc::string::String;

use shield_wire::*;

use crate::wallet::state::State;

/* An open names the private address as soon as it is derived from the words, before the
 * store it belongs to is made: shown at once, never waiting on the store or the network. */
pub(super) fn previewed(state: &mut State, op: u16, body: &str) -> bool {
    if !matches!(op, OP_OPEN_WORDS | OP_OPEN_KEY) {
        return false;
    }
    let Some(address) = field(body, "address").filter(|a| a.starts_with("nox1")) else {
        return false;
    };
    if state.shield_ui.nox1.as_deref() == Some(address) {
        return false;
    }
    state.shield_ui.nox1 = Some(String::from(address));
    true
}

/* The prover's phase and fraction, while it reports them. */
pub(super) fn progress(state: &mut State, body: &str) -> bool {
    let ui = &mut state.shield_ui;
    let phase = field(body, "phase").map(String::from);
    let permille = field(body, "fraction")
        .and_then(|f| f.parse::<f32>().ok())
        .filter(|f| f.is_finite())
        .map(|f| (f.clamp(0.0, 1.0) * 1000.0) as u16);
    let changed = phase != ui.job.phase || permille != ui.job.permille;
    ui.job.phase = phase;
    ui.job.permille = permille;
    changed
}
