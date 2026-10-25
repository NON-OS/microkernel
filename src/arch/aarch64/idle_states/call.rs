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

//! Entering one idle state through PSCI, its deadline handed to the
//! broadcast frame first when the state stops this CPU's timer.

use super::powerdown;
use crate::arch::aarch64::psci::{self, PsciError};
use crate::arch::aarch64::timer::broadcast;
use crate::arch::aarch64::timer::generic::microseconds_to_ticks;
use crate::arch::fdt::find::idle::IdleState;

/// One state to enter, as `enter::chosen` picked it for roster CPU `cpu`.
pub(super) struct Chosen {
    pub(super) cpu: usize,
    pub(super) index: u8,
    pub(super) state: IdleState,
    pub(super) standby: bool,
}

/// True when the core powered down and came back, false when CPU_SUSPEND
/// returned (a standby state, or a powerdown the core stayed up through),
/// an error when PSCI refused or the deadline could not be handed off.
pub(super) fn enter_state(chosen: &Chosen) -> Result<bool, PsciError> {
    let (cpu, state) = (chosen.cpu, &chosen.state);
    // Early by the state's exit latency, so the CPU is up by its deadline.
    let lead = microseconds_to_ticks(u64::from(state.exit_us));
    if state.timer_stops && !broadcast::hand_off(cpu, lead) {
        return Err(PsciError::NotPresent);
    }
    // A standby state keeps this CPU's context, and CPU_SUSPEND returns on
    // the wake, ignoring the entry point and context id (DEN0022). A
    // powerdown state returns through the resume path, or as a call if the
    // core stayed up.
    let entered = if chosen.standby {
        psci::cpu_suspend(u64::from(state.param), 0, 0).map(|()| false)
    } else {
        powerdown::enter(cpu, state.param)
    };
    if state.timer_stops {
        broadcast::take_back(cpu);
    }
    entered
}
