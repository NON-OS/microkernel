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
 * The wallet's side of the service's one job. A press starts a job and
 * returns; each tick asks for its result until it is done or failed, then
 * hands the answer to `apply`. While a job runs no second one is started,
 * and the screens say what is being waited on.
 */

use alloc::string::String;

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use super::client::{call, Answer};
use crate::wallet::state::shield_ui::Waiting;
use crate::wallet::state::State;

const BUSY: &str = "The shield service is still busy with the last request.";
const LOST: &str = "The shield service stopped answering, so what it was doing is not known. \
     The wallet opens the shield again by itself and reads what it holds.";
/* Asks in a row without an answer before the job is said lost: with the
 * waits below, about two minutes of a service that does not answer. */
pub const MISSES: u8 = 8;
const POLL_MAX_MS: i64 = 30_000;

/* The op a job the service names was started with, for a wallet that
 * comes back to a job it did not see begin. */
fn op_of(name: &str) -> Option<u16> {
    Some(match name {
        "open" => OP_OPEN_WORDS,
        "sync" => OP_SYNC,
        "review shield" => OP_REVIEW_SHIELD,
        "confirm" => OP_CONFIRM,
        "quote" => OP_QUOTE,
        "send" => OP_SEND,
        "withdraw" => OP_WITHDRAW,
        "follow" => OP_FOLLOW,
        "review settle" => OP_REVIEW_SELF_SETTLE,
        "take back" => OP_TAKE_BACK,
        _ => return None,
    })
}

/* The service is running `name` for this wallet already: wait on it. */
pub fn resume(state: &mut State, name: &str) -> bool {
    let Some(op) = op_of(name) else { return false };
    let ui = &mut state.shield_ui;
    ui.waiting = Some(Waiting { op, since_ms: mk_uptime_ms() });
    ui.misses = 0;
    ui.poll_at_ms = 0;
    if matches!(op, OP_SEND | OP_WITHDRAW) {
        ui.screen = crate::wallet::state::shield_ui::SHIELD_PROVING;
    }
    true
}

fn why(body: &str) -> String {
    String::from(field(body, "why").unwrap_or("The shield service refused that."))
}

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

/* One look at the running job. True when the screen should repaint. */
pub fn poll(state: &mut State) -> bool {
    let Some(w) = state.shield_ui.waiting else { return false };
    let elapsed = ((mk_uptime_ms() - w.since_ms).max(0) / 1000) as u32;
    let ticked = elapsed != state.shield_ui.elapsed_s;
    state.shield_ui.elapsed_s = elapsed;
    let now = mk_uptime_ms();
    if now < state.shield_ui.poll_at_ms {
        return ticked;
    }
    let a = match call(OP_RESULT, &[]) {
        Ok(a) => {
            state.shield_ui.misses = 0;
            state.shield_ui.poll_at_ms = 0;
            a
        }
        Err(_) => return missed(state, now) | ticked,
    };
    match field(&a.body, "state") {
        Some("done") => {
            state.shield_ui.waiting = None;
            super::apply::apply(state, w.op, &a.body);
            true
        }
        Some("failed") => {
            state.shield_ui.waiting = None;
            super::apply::failed(state, w.op, why(&a.body));
            true
        }
        /* The service restarted under us: nothing is running for this wallet,
         * and its store is shut. Said, and opened again by itself. */
        Some("none") => {
            state.shield_ui.waiting = None;
            state.shield_ui.opened = false;
            state.shield_ui.nox1 = None;
            super::open::restarted(state);
            true
        }
        _ => ticked | progress(state, &a.body) | previewed(state, w.op, &a.body),
    }
}

/* An open names the private address as soon as it is derived from the words, before the
 * store it belongs to is made: shown at once, never waiting on the store or the network. */
fn previewed(state: &mut State, op: u16, body: &str) -> bool {
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

/* A result ask the service did not answer: ask later, waiting longer each
 * time, and after MISSES in a row stop waiting and say so. */
fn missed(state: &mut State, now: i64) -> bool {
    let ui = &mut state.shield_ui;
    ui.misses = ui.misses.saturating_add(1);
    if ui.misses < MISSES {
        ui.poll_at_ms = now + (1_000i64 << ui.misses.min(5)).min(POLL_MAX_MS);
        return false;
    }
    ui.waiting = None;
    ui.misses = 0;
    ui.poll_at_ms = 0;
    ui.opened = false;
    ui.phase = None;
    ui.permille = None;
    if ui.screen == crate::wallet::state::shield_ui::SHIELD_PROVING && ui.spend.is_none() {
        ui.screen = ui.from;
    }
    super::open::restarted(state);
    state.shield_ui.failure = Some(String::from(LOST));
    true
}

/* The prover's phase and fraction, while it reports them. */
fn progress(state: &mut State, body: &str) -> bool {
    let ui = &mut state.shield_ui;
    let phase = field(body, "phase").map(String::from);
    let permille = field(body, "fraction")
        .and_then(|f| f.parse::<f32>().ok())
        .filter(|f| f.is_finite())
        .map(|f| (f.clamp(0.0, 1.0) * 1000.0) as u16);
    let changed = phase != ui.phase || permille != ui.permille;
    ui.phase = phase;
    ui.permille = permille;
    changed
}

/* Stop a proof at its next phase. The job then ends as failed. */
pub fn cancel() {
    let _ = call(OP_CANCEL, &[]);
}
