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

//! What boot keeps for the idle path: the tree's states, each CPU's list,
//! the power_state format, and the states the firmware has refused.

use core::sync::atomic::AtomicU8;

use spin::Once;

use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::fdt::find::idle::{CpuStates, IdleState, MAX_STATES};

pub(super) struct Tree {
    pub(super) states: [IdleState; MAX_STATES],
    pub(super) count: usize,
    /// By roster index, which is the order `/cpus` lists them.
    pub(super) cpus: [CpuStates; MAX_CPUS],
}

impl Tree {
    pub(super) fn states(&self) -> &[IdleState] {
        &self.states[..self.count.min(MAX_STATES)]
    }
}

/// Set once the tree is read.
pub(super) static TREE: Once<Tree> = Once::new();

/// Set once PSCI is known to take CPU_SUSPEND: whether power_state is in the
/// extended format. Until then every CPU waits on a plain WFI.
pub(super) static EXTENDED: Once<bool> = Once::new();

/// Per CPU, a bit per state the firmware refused. Kept so a refused state
/// costs one call, not one per idle.
pub(super) static REFUSED: [AtomicU8; MAX_CPUS] = [const { AtomicU8::new(0) }; MAX_CPUS];
