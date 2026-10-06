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

mod access;
mod alarm_scan;
mod build_pcb;
mod claim;
mod create;
mod current_pid;
mod inherit;
mod ops;
mod pid;
mod pid_alloc;
mod thread_spawn;
mod thread_start;
mod types;

pub(crate) use inherit::AMBIENT_CAPS;
pub use claim::{claim_new, release_new};
pub(crate) use create::create_process_with_parent;
pub use create::{create_process, create_process_with_mem};
pub use current_pid::CURRENT_PID;
pub use pid::allocate_tid;
pub use thread_spawn::{admit_thread, spawn_thread, spawn_thread_in, spawn_thread_parked};
pub(crate) use thread_start::start_in_user_half;
pub use types::{ProcessTable, PROCESS_TABLE};
