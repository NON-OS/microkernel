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

//! Opening the shield whenever it is not open or opening, and whether the
//! service already holds this account.

use super::*;

/* Open the shield for this account if it is not open or opening, so its
 * private address is there wherever it is shown. Quiet when no service
 * runs: the screen asking says so itself. */
pub fn ensure(state: &mut State) {
    if !here() || state.shield_ui.opened || state.shield_ui.waiting.is_some() || retrying(state) {
        return;
    }
    probe_then_open(state);
}

/* Look for the service and open the shield if it is there. A service not
 * up yet, as early in a boot, is looked for again by itself, waiting longer
 * each time (`later`), so the shield never stays closed for want of one look. */
pub(super) fn probe_then_open(state: &mut State) {
    state.shield = super::super::probe::probe();
    if state.shield.available() {
        open(state);
    } else {
        later(state);
    }
}

/* Whether the service already holds this account open; reads it if so. */
pub(super) fn held(
    state: &mut State,
    account: &str,
    running: &mut Option<String>,
) -> Result<bool, &'static str> {
    let a = call(OP_STATE, &[])?;
    let open = field(&a.body, "unlocked") == Some("1")
        && field(&a.body, "account").is_some_and(|x| x.eq_ignore_ascii_case(account));
    if open {
        super::super::apply_state::state(state, &a.body);
        *running = field(&a.body, "job").map(String::from);
    }
    Ok(open)
}
