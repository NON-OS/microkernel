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

//! The timers that end in a signal: ITIMER_REAL, which alarm and setitimer
//! set, and the POSIX timers timer_create makes. Deadlines are milliseconds of
//! the family's monotonic clock, the finest step it keeps.

/// ITIMER_REAL: when it next fires, and its period, 0 for once.
#[derive(Clone, Copy)]
pub struct Itimer {
    pub due: u64,
    pub interval: u64,
}

/// One timer_create timer.
#[derive(Clone, Copy)]
pub struct PosixTimer {
    /// The id the guest was given, from 0 up, as Linux numbers them.
    pub id: i32,
    /// The clock an absolute time is read on.
    pub clock: u64,
    /// The signal it raises, 0 for SIGEV_NONE.
    pub signo: u8,
    /// The thread SIGEV_THREAD_ID named, 0 for the process.
    pub tid: u32,
    /// sigev_value, handed back in si_value.
    pub value: u64,
    pub due: Option<u64>,
    pub interval: u64,
    /// Expiries while its signal was still queued, and the count the last
    /// taken signal carried, which timer_getoverrun reports.
    pub overrun: i32,
    pub last_overrun: i32,
    /// Its signal is queued and not yet taken.
    pub queued: bool,
}
