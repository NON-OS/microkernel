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

//! Reading the tree's idle states at boot, and asking PSCI how to name them.

use super::format::is_standby;
use super::powerdown;
use super::psci_format::suspend_format;
use super::table::{Tree, EXTENDED, TREE};
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::fdt::find::cpus::idle_lists;
use crate::arch::fdt::find::idle::{states, CpuStates, IdleState, Refusal, MAX_STATES};
use crate::arch::fdt::Fdt;
use crate::sys::serial::Line;

/// Keep the tree's states and each CPU's list; how many states were kept.
/// Read while the tree is, since it is gone by the time a CPU idles.
pub fn describe(fdt: &Fdt) -> Result<usize, Refusal> {
    let mut found = [IdleState::default(); MAX_STATES];
    let count = states(fdt, &mut found)?;
    let mut cpus = [CpuStates::default(); MAX_CPUS];
    idle_lists(fdt, &found[..count], &mut cpus)?;
    TREE.call_once(|| Tree { states: found, count, cpus });
    Ok(count)
}

/// Turn the states on once PSCI says CPU_SUSPEND is there and in which
/// format it reads power_state. Any doubt leaves every CPU on a plain WFI.
pub fn prepare() {
    let Some(tree) = TREE.get().filter(|t| t.count > 0) else {
        return;
    };
    let mut line = Line::new();
    line.str(b"[IDLE] ");
    let extended = suspend_format();
    match extended {
        Ok(extended) => {
            EXTENDED.call_once(|| extended);
            let down = powerdown::available();
            let usable = tree.states().iter().filter(|s| !s.timer_stops);
            let usable = usable.filter(|s| down || is_standby(s.param, extended)).count();
            line.str(b"states=").dec(tree.count as u64).str(b" usable=").dec(usable as u64);
            line.str(if extended { b" format=extended" } else { b" format=original" });
            line.str(if down { b" powerdown=resumable" } else { b" powerdown=no" });
        }
        Err(reason) => _ = line.str(reason).str(b"; WFI only"),
    }
    line.end();
}
