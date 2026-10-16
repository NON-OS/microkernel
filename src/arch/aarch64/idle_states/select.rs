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

//! Which of this CPU's states to enter now. Pure, so the proofs run it.

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

/// Whether `state` may be entered at all under `allow`: standby, so
/// CPU_SUSPEND returns here on a wake, or powerdown with the resume path;
/// and keeping the local timer, or stopping it with a broadcast frame.
pub fn allowed(state: &IdleState, extended: bool, allow: Allow) -> bool {
    (allow.timer_stop || !state.timer_stops)
        && (allow.powerdown || is_standby(state.param, extended))
}

/// The deepest state, by `min-residency-us`, that this CPU may enter now:
/// - one of its own (`list`, indices into `states`);
/// - `allowed` under `allow`;
/// - not refused by the firmware before (`refused`, a bit per index);
/// - paying for itself before the timer fires: its residency and its exit
///   latency within `budget_us`.
///
/// Ties go to the first listed. `None` means a plain WFI.
pub fn pick(
    states: &[IdleState],
    list: &[u8],
    extended: bool,
    refused: u8,
    budget_us: u64,
    allow: Allow,
) -> Option<u8> {
    let mut best: Option<(u8, u32)> = None;
    for &index in list {
        let Some(state) = states.get(usize::from(index)) else {
            continue;
        };
        // An index with no bit to record a refusal in is never entered.
        let Some(bit) = 1u8.checked_shl(u32::from(index)) else {
            continue;
        };
        let cost = u64::from(state.residency_us) + u64::from(state.exit_us);
        if !allowed(state, extended, allow) || refused & bit != 0 || cost > budget_us {
            continue;
        }
        if best.is_none_or(|(_, deepest)| state.residency_us > deepest) {
            best = Some((index, state.residency_us));
        }
    }
    best.map(|(index, _)| index)
}
