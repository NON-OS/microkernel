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

//! What the kernel keeps of one idle state, and of one CPU's list of them.
//! Free of the parser, so the selection proofs can mount it alone.

/// States kept from `/cpus/idle-states`. A tree with more is refused whole
/// rather than cut, since which ones were dropped would decide the depth.
pub const MAX_STATES: usize = 8;

/// States one CPU's `cpu-idle-states` may name.
pub const MAX_PER_CPU: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IdleState {
    /// How `cpu-idle-states` names it.
    pub phandle: u32,
    /// `arm,psci-suspend-param`: the power_state CPU_SUSPEND is given,
    /// exactly as the tree has it.
    pub param: u32,
    pub entry_us: u32,
    pub exit_us: u32,
    /// `min-residency-us`: the shortest stay that saves anything.
    pub residency_us: u32,
    /// `local-timer-stop`: this CPU's generic timer stops in the state.
    pub timer_stops: bool,
}

/// One CPU's states, as indices into the kept table, in the tree's order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CpuStates {
    pub count: u8,
    pub index: [u8; MAX_PER_CPU],
}

impl CpuStates {
    pub fn indices(&self) -> &[u8] {
        &self.index[..usize::from(self.count).min(MAX_PER_CPU)]
    }
}
