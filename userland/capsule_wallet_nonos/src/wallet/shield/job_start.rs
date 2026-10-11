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

//! A press starts a job and returns; the tick waits on it (job_poll).

use alloc::string::String;

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use super::client::{call, Answer};
use super::job::{why, BUSY};
use super::job_resume::resume;
use crate::wallet::state::shield_ui::Waiting;
use crate::wallet::state::State;

/* Ask the service to start `op`. True when it is running. */
pub fn start(state: &mut State, op: u16, fields: &[&str]) -> bool {
    let ui = &mut state.shield_ui;
    if !super::open::here() {
        ui.failure = Some(String::from(super::open::ELSEWHERE));
        return false;
    }
    if ui.waiting.is_some() {
        ui.failure = Some(String::from(BUSY));
        return false;
    }
    match call(op, fields) {
        Ok(Answer { status: STATUS_STARTED, .. }) => {
            ui.waiting = Some(Waiting { op, since_ms: mk_uptime_ms() });
            ui.misses = 0;
            ui.poll_at_ms = 0;
            ui.elapsed_s = 0;
            ui.phase = None;
            ui.permille = None;
            ui.failure = None;
            true
        }
        /* Busy with a job this wallet lost sight of: it is waited on, so
         * its answer still lands where it belongs. */
        Ok(Answer { status: STATUS_BUSY, body }) => {
            if let Some(name) = field(&body, "job") {
                resume(state, name);
            }
            state.shield_ui.failure = Some(String::from(BUSY));
            false
        }
        Ok(a) => {
            ui.failure = Some(why(&a.body));
            false
        }
        Err(e) => {
            ui.failure = Some(String::from(e));
            false
        }
    }
}
