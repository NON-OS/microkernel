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

//! The log, the serial line, the shootdown call and the CPU, as the
//! mounted files call them.

use std::cell::{Cell, RefCell};

thread_local! {
    static SHOOTDOWNS: Cell<u64> = const { Cell::new(0) };
    pub(crate) static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// A log line or a serial line, kept for the test to read.
#[macro_export]
macro_rules! log_line {
    ($($t:tt)*) => { $crate::stand_in::kernel::keep(::std::format!($($t)*)) };
}
#[macro_export]
macro_rules! log_info {
    ($($t:tt)*) => { $crate::log_line!($($t)*) };
}
#[macro_export]
macro_rules! log_warn {
    ($($t:tt)*) => { $crate::log_line!($($t)*) };
}

pub fn keep(line: String) {
    LINES.with_borrow_mut(|l| l.push(line));
}

/// The lines logged on this thread since the last call; clears them.
pub fn lines() -> Vec<String> {
    LINES.with_borrow_mut(core::mem::take)
}

/// How many times `smp::serve_shootdowns` ran on this thread.
pub fn shootdowns() -> u64 {
    SHOOTDOWNS.get()
}

pub mod log {
    pub use crate::log_line as security_log;
    pub use crate::{log_line as error, log_line as info, log_line as warn};
}

include!("kernel_cpu.rs");
