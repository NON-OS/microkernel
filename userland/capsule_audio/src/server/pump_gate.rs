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

//! The driver's stream started when the first client plays and stopped
//! when the last one stops.

use super::pump::PumpState;
use super::pump_rule::started;
use super::streams::StreamTable;
use crate::mark::mark;
use crate::sink::Sink;

pub(super) fn gate(state: &mut PumpState, table: &StreamTable, sink: &Sink) -> bool {
    if !table.any_active() {
        if state.running {
            state.pending = None;
            state.running = false;
            sink.stream_stop(state.sent as u32);
            mark("[AUDIO] dac-stop\n");
        }
        return false;
    }
    if !state.running {
        // Refused, it is asked again on the next step.
        state.running = started(sink.stream_start(state.sent as u32));
        if !state.running {
            return false;
        }
        mark("[AUDIO] dac-start\n");
    }
    true
}
