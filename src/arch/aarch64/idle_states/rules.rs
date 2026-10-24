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

//! Which kinds of idle state a CPU may enter at all. Pure, so the proofs
//! run it.

use super::format::is_standby;
use crate::arch::fdt::find::idle::IdleState;

/// What a CPU may enter beyond a standby state that keeps its timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Allow {
    /// The resume path may be used (powerdown.rs).
    pub powerdown: bool,
    /// A broadcast frame will wake it for its deadline (timer::broadcast).
    pub timer_stop: bool,
}

/// What roster CPU `cpu` may enter. The CPU the broadcast frame's interrupt
/// is aimed at (`owner`) never stops its timer or powers down: asleep, it
/// would wake no one else. `frame` and `resume` say the broadcast is up and
/// the resume path checked (resume_map.rs).
pub fn allowance(cpu: usize, owner: Option<usize>, frame: bool, resume: bool) -> Allow {
    let other = owner.is_some_and(|owner| owner != cpu);
    Allow { powerdown: resume && (other || owner.is_none()), timer_stop: frame && other }
}

/// Whether `state` may be entered at all under `allow`: standby, so
/// CPU_SUSPEND returns here on a wake, or powerdown with the resume path;
/// and keeping the local timer, or stopping it with a broadcast frame.
pub fn allowed(state: &IdleState, extended: bool, allow: Allow) -> bool {
    (allow.timer_stop || !state.timer_stops)
        && (allow.powerdown || is_standby(state.param, extended))
}
