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
mod futex_requeue;
mod futex_time;
mod ident;
mod io;
mod ioctl;
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
mod pipe_poll;
mod pipe_read;
mod sched;
mod session;
pub mod sigframe;
pub mod sigframe_build;
mod sigframe_read;
mod signal;
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
mod sigreturn;
mod sleep;
mod spawn;
mod thread;
mod timer_create;
mod timer_ops;
mod timer_set;
mod timer_sigev;
mod timeops;
mod umask;
mod uname;
mod vector;
mod vector_read;

pub use ctl::fcntl;
pub use ioctl::ioctl;
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
pub use pipe_poll::bits as pipe_bits;
pub use pipe_read::read as pipe_read;
pub use sched::{
    priority_bound, sched_getparam, sched_getscheduler, sched_setaffinity, sched_setparam,
    sched_setscheduler,
};
pub use session::{getpgid, getsid, setpgid, setsid};
pub use signal::rt_sigaction;
pub use signal_mask::rt_sigprocmask;
pub use signal_queue::{rt_sigqueueinfo, rt_tgsigqueueinfo};
pub use signal_send::{kill, kill_from, sigpipe, tgkill_from};
pub use signal_stack::sigaltstack;
pub use signal_timer::{alarm, getitimer, setitimer};
pub use signal_timedwait::rt_sigtimedwait;
pub use signal_wait::{pause, rt_sigpending, rt_sigsuspend};
pub use sigreturn::rt_sigreturn;
pub use sleep::{clock_nanosleep, nanosleep};
pub use spawn::{clone, clone_process, execve, fork, vfork, wait4, wait4_usage, waitid};
pub use spawn::{WALL, WCLONE, WEXITED, WNOHANG, WNOWAIT};
pub use clock::{clock_getres, clock_gettime, now_ms};
pub use epoch::{family_ms, mark_start};
pub use thread::{arch_prctl, getrandom};
pub use timer_create::timer_create;
pub use timer_ops::{timer_delete, timer_getoverrun, timer_gettime};
pub use timer_set::timer_settime;
pub use timeops::{gettimeofday, time};
pub use umask::{umask, DEFAULT_UMASK};
pub use uname::uname;
pub use vector::writev;
pub use vector_read::readv;
