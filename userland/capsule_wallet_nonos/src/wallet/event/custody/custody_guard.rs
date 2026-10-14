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

//! Whether a new wallet may be written over what this machine holds.
//!
//! Making, importing or restoring a wallet writes the vault anew. Over a
//! vault not read yet that would lose a wallet nobody has seen, so it is
//! refused while the read may still settle. A store silent past the
//! patience, the account open now, or a vault this boot cannot open is
//! asked twice: the first press says what may be replaced, the second goes
//! on.

use super::replace_rule::{gave_up, rule, Replace};
use crate::wallet::state::State;

/// Ok when the new wallet may be written now; the reason to show if not.
pub fn may_replace(state: &mut State) -> Result<(), &'static str> {
    let waited = gave_up(state.vault_silent_since, nonos_libc::mk_uptime_ms());
    match rule(state.vault_restore_tried, waited, state.address_ready, state.vault_present) {
        Replace::Free => Ok(()),
        Replace::Refuse(why) => {
            state.custody_armed = false;
            Err(why)
        }
        Replace::Ask(_) if state.custody_armed => {
            state.custody_armed = false;
            Ok(())
        }
        Replace::Ask(why) => {
            state.custody_armed = true;
            Err(why)
        }
    }
}
