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

//! An open's answer and a take-back's: the private address and the notes
//! made spendable again, then a sync.

use alloc::format;
use alloc::string::String;

use shield_wire::*;

use super::apply_refresh::refresh;
use super::apply_values::text;
use crate::wallet::state::State;

pub(super) fn opened(state: &mut State, body: &str) {
    state.shield_ui.opened = true;
    state.shield_ui.job.phase = None;
    /* How long each step of the open took, for whoever checks a boot. */
    if let Some(took) = field(body, "took") {
        state.shield_ui.news = Some(format!("The shield opened ({took})."));
    }
    state.shield_ui.nox1 = field(body, "address").map(String::from);
    state.shield_ui.link.live = field(body, "kept") == Some("memory");
    super::job::start(state, OP_SYNC, &[]);
}

pub(super) fn taken_back(state: &mut State, body: &str) {
    let n = text(body, "returned");
    let ui = &mut state.shield_ui;
    /* Every spend's notes came back, so none of them goes on. */
    ui.spend = None;
    ui.earlier.clear();
    ui.news = Some(format!("{n} notes are spendable again. Nothing was paid."));
    refresh(state);
    super::job::start(state, OP_SYNC, &[]);
}
