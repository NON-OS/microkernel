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

//! Driving a run from either side, and reading what the server reported.

use super::run::{Host, Run};
use super::user::User;
use nsh_client::{Client, NonosAttester};
use nsh_server::Event;

impl Run {
    /// Run a host call with the server's randomness, then pump.
    pub fn host<T>(&mut self, f: impl FnOnce(&mut Host, u64, nsh_server::Rand<'_>) -> T) -> T {
        let s_rng = &mut self.s_rng;
        let out = f(&mut self.server, self.now, &mut |b| s_rng.fill(b));
        self.server_events();
        self.pump();
        out
    }

    /// Run a client call, then pump.
    pub fn user<T>(&mut self, f: impl FnOnce(&mut Client<NonosAttester, User>) -> T) -> T {
        let out = f(&mut self.client);
        self.pump();
        out
    }

    /// The server's events of one shape.
    pub fn refused(&self) -> Vec<nsh_server::Why> {
        let why = |e: &Event| match e {
            Event::Refused(w) => Some(*w),
            _ => None,
        };
        self.s_events.iter().filter_map(why).collect()
    }

    /// The record's bytes as the server handed them to the host.
    pub fn kept_record(&self) -> Vec<u8> {
        let part = |e: &Event| match e {
            Event::Record(b) => Some(b.clone()),
            _ => None,
        };
        self.s_events.iter().filter_map(part).collect::<Vec<_>>().concat()
    }

    pub fn checkpoints(&self) -> Vec<Vec<u8>> {
        let ck = |e: &Event| match e {
            Event::Checkpoint(b) => Some(b.clone()),
            _ => None,
        };
        self.s_events.iter().filter_map(ck).collect()
    }
}
