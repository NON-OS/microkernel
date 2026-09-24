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

use core::sync::atomic::Ordering;

use super::super::types::{Pid, Priority, ProcessState};
use super::build_pcb::build_pcb;
use super::inherit::compute_inherited_caps;
use super::types::{allocate_tid, CURRENT_PID, PROCESS_TABLE};
use crate::kernel_core::process_spawn::capsule_spawn::AttestedParent;

pub fn create_process(
    name: &str,
    state: ProcessState,
    prio: Priority,
) -> Result<Pid, &'static str> {
    create_process_with_mem(name, state, prio, 0)
}

pub fn create_process_with_mem(
    name: &str,
    state: ProcessState,
    prio: Priority,
    mem_kb: u64,
) -> Result<Pid, &'static str> {
    create_process_with_parent(name, state, prio, mem_kb, None)
}

/// Same as `create_process_with_mem`, but lets a spawn site attribute the new
/// process to a kernel-attested `parent_override` pid instead of the calling
/// process.
pub(crate) fn create_process_with_parent(
    name: &str,
    state: ProcessState,
    prio: Priority,
    mem_kb: u64,
    parent_override: Option<AttestedParent>,
) -> Result<Pid, &'static str> {
    if name.is_empty() {
        return Err("empty name");
    }
    let pid = allocate_tid().ok_or("pid space exhausted")?;
    let parent_pid = parent_override
        .map(AttestedParent::pid)
        .unwrap_or_else(|| CURRENT_PID.load(Ordering::Relaxed));
    let caps = compute_inherited_caps(pid, parent_pid);
    let pcb = build_pcb(pid, parent_pid, name, state, prio, mem_kb / 4, caps)?;
    crate::process::address_space::lifecycle::allocate(&pcb)?;
    crate::process::caps::rebind_address_space(&pcb).ok_or("boot session nonce missing")?;
    PROCESS_TABLE.add(pcb);
    Ok(pid)
}
