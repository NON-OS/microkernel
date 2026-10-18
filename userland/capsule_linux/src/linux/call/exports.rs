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

//! What the call families give the rest of the personality, named at
//! `crate::linux::call` by mod.rs.

pub use super::clock::{clock_getres, clock_gettime, now_ms};
pub use super::console::carry as console_carry;
pub use super::ctl::fcntl;
pub use super::cwd::{chdir, fchdir, getcwd};
pub use super::epoch::{family_ms, mark_start};
pub use super::futex::futex;
pub use super::glibc::prctl;
pub use super::glibc_sched::{clone3, getcpu, membarrier, sched_getaffinity};
pub use super::ident::{getppid, setuid};
pub use super::io::{close, read, write};
pub use super::ioctl::ioctl;
pub(crate) use super::ioctl::stream as console_stream;
pub use super::iovec::iovecs;
pub use super::life::{exit, exit_thread, killed, set_tid_address};
pub use super::life_one::exit_one;
pub use super::limits::calls::{getrlimit, prlimit64};
pub use super::limits::table::limit_for;
pub use super::mem::{brk, mmap, mprotect, mremap, munmap, MapReq};
pub use super::pipe::dup::{dup, dup2};
pub use super::pipe::make::pipe2;
pub use super::pipe::poll::bits as pipe_bits;
pub use super::process::*;
pub use super::sched::{
    priority_bound, sched_getparam, sched_getscheduler, sched_setaffinity, sched_setparam,
    sched_setscheduler,
};
pub use super::session::{getpgid, getsid, setpgid, setsid};
pub use super::signal::rt_sigaction;
pub use super::signal_act::wait_mask;
pub use super::signal_mask::rt_sigprocmask;
pub use super::signal_queue::{rt_sigqueueinfo, rt_tgsigqueueinfo};
pub use super::signal_send::{kill, kill_from, sigpipe, tgkill_from};
pub use super::signal_stack::sigaltstack;
pub use super::signal_timedwait::rt_sigtimedwait;
pub use super::signal_timer::{alarm, getitimer, setitimer};
pub use super::signal_wait::{pause, rt_sigpending, rt_sigsuspend};
pub use super::signalfd::open::signalfd4;
pub use super::signalfd::read::signalfd_read;
pub use super::signalfd::take::{signalfd_bits, signalfd_now, signalfd_take};
pub use super::sigreturn::rt_sigreturn;
pub use super::sleep::{clock_nanosleep, nanosleep};
pub use super::spawn::{clone, clone_process, execve, fork, vfork, wait4, wait4_usage, waitid};
pub use super::spawn::{tasks, WALL, WCLONE, WEXITED, WNOHANG, WNOWAIT};
pub use super::thread::{arch_prctl, getrandom};
pub use super::timeops::{gettimeofday, time};
pub use super::timer::create::timer_create;
pub use super::timer::ops::{timer_delete, timer_getoverrun, timer_gettime};
pub use super::timer::set::timer_settime;
pub use super::umask::{umask, DEFAULT_UMASK};
pub use super::uname::uname;
pub use super::vector::read::readv;
pub use super::vector::write::writev;
