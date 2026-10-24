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

//! Keeping the driver fed: the stream started and stopped with the clients,
//! and periods written ahead of the driver.

use super::proto::E_AGAIN;
use super::pump_gate::gate;
use super::pump_mix::{mark_progress, mix, to_bytes};
use super::pump_rule::still_running;
pub use super::pump_state::PumpState;
use super::streams::StreamTable;
use crate::sink::Sink;

pub const PERIOD_FRAMES: usize = 1024;
pub const PERIOD_SAMPLES: usize = PERIOD_FRAMES * 2;
pub const KEEP_AHEAD: usize = 3;

/// Keep the driver fed, every period mixed at the master volume. A period
/// already mixed and waiting on a full driver keeps the gain it was mixed at;
/// the next one takes the new one, a period (about 21 ms) later.
pub fn step(state: &mut PumpState, table: &mut StreamTable, sink: &Sink) {
    let gain = state.gain();
    if !gate(state, table, sink) {
        return;
    }
    if let Some(pending) = state.pending {
        let status = sink.write_pcm_status(&to_bytes(&pending), state.sent as u32);
        if status == E_AGAIN {
            return;
        }
        if !still_running(status) {
            // Kept, to go out first once the stream is started again.
            (state.running, state.lost) = (false, true);
            return;
        }
        state.pending = None;
        state.sent += 1;
        mark_progress(state.sent);
    }
    for _ in 0..KEEP_AHEAD {
        let samples = mix(&mut state.mixer, table, gain);
        let status = sink.write_pcm_status(&to_bytes(&samples), state.sent as u32);
        if status == E_AGAIN || !still_running(status) {
            state.running = still_running(status);
            state.lost |= !state.running;
            state.pending = Some(samples);
            break;
        }
        if status < 0 {
            break;
        }
        state.sent += 1;
        mark_progress(state.sent);
    }
}
