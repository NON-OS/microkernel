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

//! One hosted process, and everything this personality remembers about it.

use alloc::vec::Vec;

use super::fd::Fd;

pub struct Guest {
    pub pid: u32,
    /// The program break, as `brk` moves it.
    pub brk: u64,
    /// The next address an anonymous mapping gets, growing upward.
    pub mmap_next: u64,
    pub fds: Vec<Fd>,
    /// Every span backed for the guest, in order; fork copies exactly this.
    pub regions: Vec<crate::linux::guest::Region>,
    /// Pipe buffers, named by index from the descriptors at each end.
    pub pipes: Vec<Vec<u8>>,
    /// For each pipe, whether a read end and a write end are open anywhere
    /// in the family. Filled when the family lends the buffers.
    pub pipe_ends: Vec<(bool, bool)>,
    /// eventfd counters, named by index from their descriptors. The
    /// family's, lent with the pipes.
    pub events: Vec<super::Event>,
    /// timerfd timers, the same way.
    pub timers: Vec<super::Timer>,
    /// Children this guest has forked, for wait to report on.
    pub children: Vec<u32>,
    /// Tids of this guest's threads, not counting itself.
    pub threads: Vec<u32>,
    /// The word each thread asked to have cleared when it exits, from
    /// CLONE_CHILD_CLEARTID or set_tid_address: zeroed and woken then,
    /// which is what a joiner waits for.
    pub clear_tids: Vec<(u32, u64)>,
    /// Threads parked in a futex wait, with the word they wait on.
    pub waits: Vec<(u32, u64)>,
    /// The futex waits that have a timeout: the monotonic deadline, and who.
    pub futex_until: Vec<(u64, u32)>,
    /// The display connection, when the guest has opened one.
    pub display: crate::linux::unix::Conn,
    /// The Wayland objects that connection has created.
    pub objects: crate::linux::wayland::Objects,
    /// What those objects describe, and the surface it reaches.
    pub scene: crate::linux::wayland::Scene,
    /// Signal dispositions and what is raised against this process's threads.
    pub signals: super::sigqueue::Signals,
    /// What a relative path is relative to.
    pub cwd: Vec<u8>,
    /// Names this guest has resolved, each with the address it was given.
    pub automap: Vec<(Vec<u8>, [u8; 4])>,
    /// Where the guest last asked its thread pointer to be set.
    pub fs_base: u64,
    /// Set once the guest asks to end, so the loop can drop it.
    pub exited: Option<i32>,
    /// The personality, which hosts every guest it spawns.
    pub parent: u32,
    /// Process group and session.
    pub pgid: u32,
    pub sid: u32,
    /// Remembered, not enforced: the store does not apply it to a new file.
    pub umask: u16,
    /// Children forked while answering, for the serve loop to adopt.
    pub forked: Vec<Guest>,
    /// Children that have ended, with their exit codes, until waited for.
    pub ended: Vec<(u32, i32)>,
    /// A parked wait4: the pid it wants, where the status goes, the caller.
    pub waiting: Option<(u64, u64, u32)>,
    /// Threads parked in a sleep: the monotonic deadline, and who.
    pub sleepers: Vec<(u64, u32)>,
    /// Calls parked until a descriptor they wait on is ready.
    pub blocked: Vec<super::Blocked>,
    /// The image's symbolic links, read once and shared by the family.
    pub links: alloc::rc::Rc<super::Links>,
    /// Lets go of this process's family sockets when it is dropped (net::sock).
    pub sockets: crate::linux::net::sock::Holder,
}
