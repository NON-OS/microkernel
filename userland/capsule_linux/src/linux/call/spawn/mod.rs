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

//! Making and replacing processes: clone, fork, exec, wait.

mod clone;
mod exec;
mod exec_args;
mod exec_clear;
mod exec_load;
mod exec_resolve;
mod exec_shebang;
mod exec_threads;
mod fork;
mod fork_child;
mod fork_copy;
mod vfork;
mod vfork_clone;
mod vfork_flags;
mod wait;
mod waitid;

pub use clone::clone;
pub use exec::execve;
pub use fork::fork;
pub use vfork::vfork;
pub use vfork_clone::clone_process;
pub use wait::{wait4, wait4_usage, WALL, WCLONE, WEXITED, WNOHANG, WNOWAIT};
pub use waitid::waitid;
