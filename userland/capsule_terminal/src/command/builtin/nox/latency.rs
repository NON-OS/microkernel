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

//! `latency`: how long keys take to reach the screen, over the last keys
//! typed in this tab, from the event's arrival to its frame being painted.

use alloc::format;

use crate::command::output::Output;
use crate::term::state::State;

pub fn run(state: &mut State) {
    let said = match state.key_lat.spread() {
        Some(s) => format!(
            "keys to screen, last {}: middle {} ms, 95% {} ms, worst {} ms",
            s.keys, s.p50, s.p95, s.max
        ),
        None => "keys to screen: no key timed yet in this tab".into(),
    };
    Output::new(&mut state.scrollback).writeln(said.as_bytes());
}
