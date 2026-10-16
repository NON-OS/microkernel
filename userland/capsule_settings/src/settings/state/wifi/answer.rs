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

//! The window thread's side of a scan or join on the worker: handing it
//! over, and putting the answer on the panel.

use super::super::state::State;
use super::apply::apply;
use super::pending::{Pending, Work};
use super::worker::{perform, send, Ask, Sent};
use alloc::boxed::Box;

/// What the panel says when a request comes while the last is out.
const STILL_OUT: &str = "Still waiting for the driver's last answer; try again when it comes";

/// What the panel says after a request ran on the window thread.
const WAITED: &str = "No worker thread could start; the window waited for the driver";

/// Hand `ask` to the worker. The request stays on the panel ("Scanning...",
/// "Joining <name>...") until a tick finds the answer. With no worker to be
/// had, it runs here, as it did before workers, and the panel says so.
pub(super) fn hand_off(state: &mut State, work: Work, ask: Ask) {
    match send(Box::new(ask)) {
        Sent::Out => state.wifi.pending = Pending::Out(work),
        Sent::Busy => state.wifi.notice = Some(STILL_OUT),
        Sent::Refused(ask) => {
            let answer = perform(&ask);
            drop(ask);
            apply(state, answer);
            if state.wifi.notice.is_none() {
                state.wifi.notice = Some(WAITED);
            }
        }
    }
}

/// While a request is out the driver is busy with it: refuse another, and
/// say why. True when refused.
pub fn refuse_while_out(state: &mut State) -> bool {
    if !state.wifi.pending.is_out() {
        return false;
    }
    state.wifi.notice = Some(STILL_OUT);
    true
}
