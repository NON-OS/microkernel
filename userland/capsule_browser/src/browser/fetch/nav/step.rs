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

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::run::run;
use crate::browser::state::State;

/// Step the navigation as far as it will go before `until`, keep the status
/// line on its phase, and land it in the call it ends. True when it landed.
pub(in crate::browser::fetch) fn step(state: &mut State, w: &mut NetWire, until: i64) -> bool {
    let Some(f) = state.fetch.as_mut() else { return false };
    run(w, f, until);
    if !f.ended() {
        state.status = crate::browser::fetch::progress::status(f);
        return false;
    }
    let Some(job) = state.fetch.take() else { return false };
    super::commit_nav::land_nav(state, w, job)
}
