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

//! One file per family of Linux calls; declarations and re-exports only.

mod clock;
mod console;
mod ctl;
mod cwd;
mod epoch;
mod exports;
mod futex;
mod futex_op;
mod futex_requeue;
mod futex_time;
mod glibc;
mod glibc_sched;
mod ident;
mod io;
mod io_socket;
mod ioctl;
mod ioctl_queued;
mod ioctl_req;
mod ioctl_tty;
mod iovec;
mod life;
mod life_one;
mod limits;
pub mod mem;
mod pipe;
mod process;
mod random_flags;
mod sched;
mod session;
pub mod sigframe;
pub mod sigframe_build;
mod sigframe_read;
mod signal;
mod signal_act;
mod signal_its;
mod signal_itv;
mod signal_mask;
mod signal_post;
mod signal_queue;
mod signal_real;
mod signal_send;
mod signal_stack;
mod signal_timedwait;
mod signal_timer;
mod signal_wait;
mod signalfd;
mod sigreturn;
mod sleep;
mod spawn;
mod thread;
mod timeops;
mod timer;
mod umask;
mod uname;
mod vector;

pub use exports::*;
