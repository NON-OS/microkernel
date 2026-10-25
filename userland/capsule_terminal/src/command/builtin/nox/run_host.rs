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
//! Run a nox command about this machine; returns whether it succeeded.

use super::route::host::Host;
use super::{battery, caps, date, id, kill, latency, svc, sysinfo, uptime};
use crate::command::output::Output;
use crate::term::state::State;

pub(super) fn run(state: &mut State, cmd: Host, args: &[&[u8]], rest: &[&[u8]]) -> bool {
    match cmd {
        Host::Date => date::run(state),
        Host::Kill => kill::run(state, rest),
        Host::Uptime => {
            uptime::run(state);
            true
        }
        Host::Latency => {
            latency::run(state);
            true
        }
        Host::Battery => {
            battery::run(state);
            true
        }
        Host::Caps => {
            caps::run(state);
            true
        }
        Host::Ps => {
            crate::command::builtin::ps::run(&mut Output::new(&mut state.scrollback));
            true
        }
        Host::Svc => {
            svc::run(state, args);
            true
        }
        Host::Id => {
            id::run(state);
            true
        }
        Host::Sys => {
            sysinfo::run(state);
            true
        }
    }
}
