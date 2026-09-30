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

//! Hosting code the kernel does not trust and does not understand.

mod clear;
mod enrol;
mod exec;
mod exec_context;
mod exec_enter;
mod fork;
mod frame;
mod frame_cpu;
mod frame_snapshot;
mod notice;
mod peer_chunk;
mod peer_copy;
mod peer_guard;
mod peer_lock;
mod peer_map;
mod peer_protect;
mod peer_tls;
mod peer_unmap;
mod registry;
mod resume;
mod signal_call;
mod signal_enter;
mod signal_regs;
mod spawn;
mod spawn_start;
mod start_context;
mod thread;
mod trap;
mod trap_claim;
mod trap_frame;
mod trap_reply;
mod trap_table;
mod trap_wait;
mod wait;

pub use clear::clear;
pub use exec::sys_foreign_exec;
pub use fork::sys_foreign_fork;
pub use frame::ForeignFrame;
pub use frame_snapshot::FRAME_WORDS;
pub use peer_copy::sys_peer_copy;
pub use peer_map::sys_peer_map;
pub use peer_protect::sys_peer_protect;
pub use peer_tls::sys_peer_tls;
pub use peer_unmap::sys_peer_unmap;
pub use registry::{is_foreign, supervisor_of};

/// Report to its supervisor that a guest thread ended on a signal, if it is
/// a guest at all. The supervisor's personality decides what follows.
pub fn note_signal_death(pid: u32, code: i32) {
    if let Some(supervisor) = registry::supervisor_of(pid) {
        notice::post(supervisor, pid, code);
    }
}
pub use signal_call::{sys_foreign_context, sys_foreign_signal};
pub use spawn::sys_foreign_spawn;
pub use spawn_start::sys_foreign_start;
pub use thread::sys_foreign_thread;
pub use trap::redirect;
pub use trap_reply::sys_foreign_reply;
pub use wait::sys_foreign_wait;
