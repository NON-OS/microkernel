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

use nonos_app_skeleton::EventOutcome;

use super::rows::{ROW_BINDING, ROW_CONFIRM, ROW_INTENT};
use super::sign_row::sign;
use crate::wallet::publish::load;
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
    (p.entry, p.confirmed, p.news) = (None, false, None);
    (p.views, p.attest_ask, p.attest_digest) = (Default::default(), None, None);
    match load(state, &state.publisher.path.clone()) {
        Ok(e) => state.publisher.entry = Some(e),
        Err(why) => state.failure = Some(why),
    }
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    if state.publisher.sending.is_some() {
        return super::click_send::click(state, press);
    }
    if let Some(done) = super::click_attest::attest(state, press, sign) {
        return done;
    }
    match press {
        Press::Back => state.view = VIEW_SETTINGS,
        Press::Dismiss => state.failure = None,
        Press::Footer(0) => return read(state),
        Press::Field(n) => state.publisher.focus = n,
        Press::Row(ROW_CONFIRM) => state.publisher.confirmed = !state.publisher.confirmed,
        Press::Row(ROW_BINDING) if state.publisher.confirmed => sign(state, ROW_BINDING),
        Press::Row(r) if r >= ROW_INTENT => super::click_acts::press(state, r),
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
