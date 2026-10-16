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
use super::table::{Tree, EXTENDED, TREE};
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::aarch64::psci;
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
    let version = psci::psci_version();
    let extended =
        if !psci::is_present() || !version.answered() || (version.major, version.minor) < (0, 2) {
            // PSCI 0.1 numbers its calls from the tree and has no version call;
            // CPU_SUSPEND is not reached that way here.
            Err(&b"no PSCI 0.2 or later to enter them through"[..])
        } else if version.supports_features() {
            // 0.2 has no PSCI_FEATURES and always the original format; from 1.0
            // CPU_SUSPEND may be absent, and its flags say which format.
            match psci::cpu_suspend_features() {
                Ok(flags) => Ok(flags & 0b10 != 0),
                Err(_) => Err(&b"PSCI does not offer CPU_SUSPEND"[..]),
            }
        } else {
            Ok(false)
        };
    match extended {
        Ok(extended) => {
            EXTENDED.call_once(|| extended);
            let usable = tree.states().iter();
            let usable = usable.filter(|s| !s.timer_stops && is_standby(s.param, extended));
            line.str(b"states=").dec(tree.count as u64).str(b" usable=").dec(usable.count() as u64);
            line.str(if extended { b" format=extended" } else { b" format=original" });
        }
        Err(reason) => _ = line.str(reason).str(b"; WFI only"),
    }
    line.end();
}
