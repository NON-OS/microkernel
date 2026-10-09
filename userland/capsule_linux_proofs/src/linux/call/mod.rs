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

//! The argument rules of the capsule's calls, each a pure file of its own.

#[path = "../../../../capsule_linux/src/linux/call/spawn/clone_flags.rs"]
pub mod clone_flags;
#[path = "../../../../capsule_linux/src/linux/call/spawn/exec_args.rs"]
pub mod exec_args;
#[path = "../../../../capsule_linux/src/linux/call/futex_op.rs"]
pub mod futex_op;
#[path = "../../../../capsule_linux/src/linux/call/ioctl_req.rs"]
pub mod ioctl_req;
#[path = "../../../../capsule_linux/src/linux/call/iovec.rs"]
pub mod iovec;
pub mod mem;
#[path = "../../../../capsule_linux/src/linux/call/random_flags.rs"]
pub mod random_flags;
#[path = "../../../../capsule_linux/src/linux/call/signal_act.rs"]
pub mod signal_act;
#[path = "../../../../capsule_linux/src/linux/call/spawn/tasks.rs"]
pub mod tasks;
#[path = "../../../../capsule_linux/src/linux/call/spawn/wait_opts.rs"]
pub mod wait_opts;
