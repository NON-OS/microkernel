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

//! Signals raised against a process and not yet taken, with the disposition
//! of each. An entry names the thread it is for, or 0 for the process as a
//! whole, which any thread not blocking it may take, as on Linux.

use alloc::vec::Vec;

use super::siginfo::SigInfo;
use super::sigstate::{SigAction, NSIG};
use super::sigthread::ThreadSig;
use super::sigtimer::{Itimer, PosixTimer};
use super::sigwaits::{ChildWait, Outbound, SigWait};

/// Linux's default RLIMIT_SIGPENDING for a small machine: how many queued
/// realtime signals a process may hold before sigqueue answers EAGAIN.
pub const QUEUE_MAX: usize = 1024;

#[derive(Clone)]
pub struct Signals {
    pub(super) actions: [SigAction; NSIG],
    pub(super) pending: Vec<(u32, SigInfo)>,
    /// Each thread's mask, alternate stack and suspended mask.
    pub(super) threads: Vec<ThreadSig>,
    /// Signals for other processes of the family, and who sent each.
    pub outbox: Vec<Outbound>,
    /// ITIMER_REAL, and the POSIX timers timer_create made.
    pub real: Option<Itimer>,
    pub timers: Vec<PosixTimer>,
    /// Threads parked in pause, sigsuspend or sigtimedwait.
    pub sigwaits: Vec<SigWait>,
    /// Threads parked in wait4 or waitid.
    pub childwaits: Vec<ChildWait>,
    /// Set once the leader has made a plain exit while other threads run on.
    pub leader_gone: bool,
    /// A vfork parent's thread, parked until this child execs or ends.
    pub vfork: Option<u32>,
    /// What this process raises at its parent when it ends; clone names it.
    pub exit_signal: u8,
    /// Children that raise something other than SIGCHLD, for __WCLONE.
    pub clone_kids: Vec<u32>,
    /// The process group each ended child was in, for a wait by group.
    pub kid_groups: Vec<(u32, u32)>,
    /// The mask of each signalfd, named by its descriptor's handle.
    pub sigfds: Vec<u64>,
}
