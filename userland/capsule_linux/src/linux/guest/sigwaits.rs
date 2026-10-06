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

//! Threads parked until a signal or a child says something, and signals on
//! their way to another process of the family.

use super::siginfo::SigInfo;

/// A thread in pause, sigsuspend or sigtimedwait.
#[derive(Clone, Copy)]
pub struct SigWait {
    pub tid: u32,
    /// sigtimedwait's set: a signal in it is taken, not handled. 0 for pause
    /// and sigsuspend, which only a handler ends.
    pub set: u64,
    /// Where sigtimedwait writes the siginfo; 0 writes none.
    pub info: u64,
    /// When sigtimedwait gives up with EAGAIN.
    pub due: Option<u64>,
    /// For a signalfd read, how many records fit where `info` points; 0 else.
    pub records: u64,
}

/// Which children a wait4 or waitid asks about.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Which {
    Any,
    Pid(u32),
    Group(u32),
}

/// A thread in wait4 or waitid, and where its answer goes.
#[derive(Clone, Copy)]
pub struct ChildWait {
    pub tid: u32,
    pub which: Which,
    pub options: u64,
    /// wait4's status word, or waitid's siginfo; and the rusage, 0 for none.
    pub out: u64,
    pub rusage: u64,
    pub waitid: bool,
}

/// Who a signal leaving this process is for.
#[derive(Clone, Copy)]
pub enum Target {
    Process(u32),
    /// A thread, and the process it must belong to, 0 for any.
    Thread(u32, u32),
    Group(u32),
    /// kill(-1): every process of the family but the sender.
    All,
}

/// A signal for another process, and the thread parked until it is sent.
#[derive(Clone, Copy)]
pub struct Outbound {
    pub from: u32,
    pub to: Target,
    pub info: SigInfo,
}
