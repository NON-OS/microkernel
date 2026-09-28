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

mod console;
mod ctl;
mod clock;
mod epoch;
mod cwd;
mod futex;
mod ident;
mod io;
mod io_socket;
mod life;
mod life_one;
mod limits;
mod limits_table;
mod glibc;
mod glibc_sched;
mod mem;
mod pipe;
mod pipe_dup;
mod pipe_end;
mod pipe_io;
mod pipe_read;
mod pipe_wait;
mod session;
pub mod sigframe;
pub mod sigframe_build;
mod sigframe_read;
mod signal;
mod signal_mask;
mod signal_post;
mod signal_queue;
mod signal_send;
mod signal_stack;
mod sigreturn;
mod sleep;
mod spawn;
mod thread;
mod timeops;
mod umask;
mod uname;
mod vector;
mod vector_read;

pub use ctl::{fcntl, ioctl};
pub use cwd::{chdir, fchdir, getcwd};
pub use futex::futex;
pub use ident::{getppid, setuid};
pub use io::{close, read, write};
pub use life::{exit, exit_thread, killed, set_tid_address};
pub use life_one::exit_one;
pub use limits::{getrlimit, prlimit64};
pub use glibc::prctl;
pub use glibc_sched::{clone3, getcpu, membarrier, sched_getaffinity};
pub use mem::{brk, mmap, mprotect, mremap, munmap, MapReq};
pub use pipe::pipe2;
pub use pipe_dup::{dup, dup2};
pub use pipe_io::write as pipe_write;
pub use pipe_read::read as pipe_read;
pub use pipe_wait::{is_pipe, read_or_park as pipe_read_or_park};
pub use session::{getpgid, getsid, setpgid, setsid};
pub use signal::rt_sigaction;
pub use signal_mask::rt_sigprocmask;
pub use signal_queue::{rt_sigqueueinfo, rt_tgsigqueueinfo};
pub use signal_send::{kill, kill_from, tgkill_from};
pub use signal_stack::sigaltstack;
pub use sigreturn::rt_sigreturn;
pub use sleep::{clock_nanosleep, nanosleep};
pub use spawn::{clone, clone_process, execve, fork, reap_one, vfork, wait4};
pub use clock::{clock_getres, clock_gettime, now_ms};
pub use epoch::{family_ms, mark_start};
pub use thread::{arch_prctl, getrandom};
pub use timeops::{gettimeofday, time};
pub use umask::{umask, DEFAULT_UMASK};
pub use uname::uname;
pub use vector::writev;
pub use vector_read::readv;
