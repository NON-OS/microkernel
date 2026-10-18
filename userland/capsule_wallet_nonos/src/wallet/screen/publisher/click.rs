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

use alloc::format;

use nonos_app_skeleton::EventOutcome;

use super::rows::{ROW_ATTEST, ROW_BINDING, ROW_CONFIRM, ROW_INTENT};
use crate::wallet::chain;
use crate::wallet::publish::{load, sign_attest, sign_binding};
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_PUBLISHER, VIEW_SETTINGS};

pub fn open(state: &mut State) -> EventOutcome {
    state.view = VIEW_PUBLISHER;
    state.scroll = 0;
    EventOutcome::Repaint
}

/// Open: read the entry document the path names; a new one starts unconfirmed.
pub(super) fn read(state: &mut State) -> EventOutcome {
    let p = &mut state.publisher;
    (p.entry, p.confirmed, p.news, p.views) = (None, false, None, Default::default());
    match load(state, &state.publisher.path.clone()) {
        Ok(e) => state.publisher.entry = Some(e),
        Err(why) => state.failure = Some(why),
    }
    EventOutcome::Repaint
}

fn sign(state: &mut State, row: u8) {
    let Some(e) = state.publisher.entry.clone() else { return };
    let path = state.publisher.path.clone();
    let chain = chain::current().id;
    let done = match row {
        ROW_BINDING => sign_binding(state, &path, &e, chain),
        r => sign_attest(state, &path, &e, (r - ROW_ATTEST) as usize, chain),
    };
    match done {
        Ok(at) => state.publisher.news = Some(format!("written to {at}")),
        Err(why) => state.failure = Some(why),
    }
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    if state.publisher.sending.is_some() {
        return super::click_send::click(state, press);
    }
    match press {
        Press::Back => state.view = VIEW_SETTINGS,
        Press::Dismiss => state.failure = None,
        Press::Footer(0) => return read(state),
        Press::Field(n) => state.publisher.focus = n,
        Press::Row(ROW_CONFIRM) => state.publisher.confirmed = !state.publisher.confirmed,
        Press::Row(ROW_BINDING) if state.publisher.confirmed => sign(state, ROW_BINDING),
        Press::Row(r) if (ROW_ATTEST..ROW_INTENT).contains(&r) => sign(state, r),
        Press::Row(r) if r >= ROW_INTENT => super::click_acts::press(state, r),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
