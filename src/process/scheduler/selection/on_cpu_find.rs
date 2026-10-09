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

//! Finding the CPU that runs a pid or is still on its stack.

use core::sync::atomic::Ordering;

use super::on_cpu::{named_by, OWNED, TRACKED};
use crate::smp::MAX_CPUS;

/// The CPU running `pid` or still on its stack, if any. For teardown paths
/// that must not free a stack or a page table a CPU is still using.
pub fn cpu_holding(pid: u32) -> Option<usize> {
    if !TRACKED || pid == 0 {
        return None;
    }
    (0..crate::smp::cpu_count().min(MAX_CPUS)).find(|&cpu| named_by(cpu, pid))
}

/// The CPU that has `pid` as its own, if any: the one to tell when `pid` is
/// killed, as a CPU only leaving it is off it by its next tick anyway.
pub fn cpu_running(pid: u32) -> Option<usize> {
    if !TRACKED || pid == 0 {
        return None;
    }
    (0..crate::smp::cpu_count().min(MAX_CPUS)).find(|&cpu| OWNED[cpu].load(Ordering::SeqCst) == pid)
}
