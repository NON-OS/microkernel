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

use core::cell::Cell;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::sys::sync::IrqMutex;

use super::line::{render, Text};
use super::phase::Phase;
use super::record::Record;
use super::table::Table;

/*
 * Launches waiting on their first syscall, surface or frame. The lock masks
 * interrupts: an exit can be reached from a fault while a syscall on the
 * same CPU holds it. `WAITING` lets the syscall path skip the lock when no
 * launch is outstanding, which is nearly always.
 */
static PENDING: IrqMutex<Table<16>> = IrqMutex::new(Table::new());
static WAITING: AtomicUsize = AtomicUsize::new(0);

fn now() -> u64 {
    crate::time::now_ns()
}

fn emit(r: &Record, end: &[u8]) {
    let mut t = Text::new();
    render(r, end, &mut t);
    crate::sys::serial::println(t.as_bytes());
}

/*
 * The spawn call's own record, on its stack. Marks go through `&self` so
 * the install parameters can carry it by shared reference. A launch that
 * never queues a pid is written out when the handle drops.
 */
pub(crate) struct Trace {
    record: Cell<Record>,
    handed_off: Cell<bool>,
}

impl Trace {
    pub(crate) fn begin(name: &str) -> Self {
        Self { record: Cell::new(Record::begin(name.as_bytes(), now())), handed_off: Cell::new(false) }
    }

    pub(crate) fn mark(&self, phase: Phase) {
        let mut r = self.record.get();
        r.mark(phase, now());
        self.record.set(r);
    }

    pub(crate) fn queued(&self, pid: u32) {
        let mut r = self.record.get();
        r.set_pid(pid);
        r.mark(Phase::Queued, now());
        self.record.set(r);
        self.handed_off.set(true);
        let displaced = {
            let mut table = PENDING.lock();
            let out = table.insert(r);
            WAITING.store(table.len(), Ordering::Release);
            out
        };
        if let Some(old) = displaced {
            emit(&old, b"displaced");
        }
    }
}

impl Drop for Trace {
    fn drop(&mut self) {
        if !self.handed_off.get() {
            emit(&self.record.get(), b"refused");
        }
    }
}

fn mark(pid: u32, phase: Phase) {
    if WAITING.load(Ordering::Acquire) == 0 {
        return;
    }
    let done = {
        let mut table = PENDING.lock();
        let out = table.mark(pid, phase, now());
        WAITING.store(table.len(), Ordering::Release);
        out
    };
    if let Some(r) = done {
        emit(&r, b"frame");
    }
}

pub(crate) fn on_syscall(pid: u32) {
    mark(pid, Phase::FirstSyscall);
}

pub(crate) fn on_surface(pid: u32) {
    mark(pid, Phase::FirstSurface);
}

pub(crate) fn on_frame(pid: u32) {
    mark(pid, Phase::FirstFrame);
}

pub(crate) fn on_exit(pid: u32) {
    if WAITING.load(Ordering::Acquire) == 0 {
        return;
    }
    let gone = {
        let mut table = PENDING.lock();
        let out = table.take(pid);
        WAITING.store(table.len(), Ordering::Release);
        out
    };
    if let Some(r) = gone {
        emit(&r, b"exit");
    }
}
