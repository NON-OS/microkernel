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

//! The interrupt line's state on this test thread: its sequence, the
//! script that answers each wait, and what the driver did with it.

use std::cell::{Cell, RefCell};

/// One wait as the script answers it: how long it took, and whether the
/// line fired in that time.
pub struct Wake {
    pub after_ms: u64,
    pub fired: bool,
}

type Script = Box<dyn FnMut(u64) -> Wake>;

thread_local! {
    pub(crate) static SEQ: Cell<u64> = const { Cell::new(0) };
    pub(crate) static WAITS: Cell<u64> = const { Cell::new(0) };
    pub(crate) static ACKS: Cell<u64> = const { Cell::new(0) };
    static SCRIPT: RefCell<Option<Script>> = const { RefCell::new(None) };
}

/// Answer every later wait with `script`, given the wait's timeout.
pub fn on_wait(script: impl FnMut(u64) -> Wake + 'static) {
    SCRIPT.with(|s| *s.borrow_mut() = Some(Box::new(script)));
}

/// The script's answer to a wait of `timeout_ms`: a quiet line, slept out
/// whole, when no script is set.
pub(crate) fn answer(timeout_ms: u64) -> Wake {
    let quiet = Wake { after_ms: timeout_ms, fired: false };
    SCRIPT.with(|s| s.borrow_mut().as_mut().map(|f| f(timeout_ms))).unwrap_or(quiet)
}

/// Waits made, and acks given, on this thread so far.
pub fn waits() -> u64 {
    WAITS.with(Cell::get)
}

pub fn acks() -> u64 {
    ACKS.with(Cell::get)
}
