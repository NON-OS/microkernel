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

//! A process's surfaces are given up when its last live thread ends (L36).
//! Teardown and finalize both ask, as `devices.rs` does for claims (L46):
//! asked only at teardown, with zombies counted as alive, two threads ending
//! together each saw the other and neither gave them up, and the tables were
//! then freed under the compositor's view. A second pass finds nothing.

use core::sync::atomic::Ordering;

use crate::kernel_core::surface_registry::pin::exit::keep_at_exit;
use crate::kernel_core::surface_registry::release_owned_by_pid;
use crate::process::core::{Pid, ProcessControlBlock};

pub(super) fn give_up_if_last(pid: Pid, pcb: &ProcessControlBlock) {
    if !super::devices::last_of_group(pid) {
        return;
    }
    let group = crate::hardware::broker::holder_of(pid);
    keep_at_exit(group, pcb.cr3.load(Ordering::Acquire));
    release_owned_by_pid(group);
}
