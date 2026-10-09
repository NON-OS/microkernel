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

//! A BAR0 window in host memory, and a part running against it on its own
//! thread where a handshake needs one.

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use nonos_devmodel::{run, FakeBar, LiveDevice};

use crate::regs::Regs;

/// Covers every register the driver touches, the transmit queue at 0xE028
/// being the highest.
pub fn window() -> Arc<FakeBar> {
    Arc::new(FakeBar::new(0x10000))
}

pub fn regs(bar: &FakeBar) -> Regs {
    Regs::new(bar.base())
}

static TURN: Mutex<()> = Mutex::new(());

/// A running part and the turn it holds. The driver's waits are on the clock,
/// but a model thread preempted past a 100 ms MDIC bound on a loaded runner
/// would read as a silent part, so one live test runs at a time. Fields drop
/// in order: the part stops before the turn is given up.
pub struct Live {
    _part: LiveDevice,
    _turn: MutexGuard<'static, ()>,
}

pub fn live<F>(bar: &Arc<FakeBar>, part: F) -> Live
where
    F: Fn(&FakeBar) + Send + 'static,
{
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    Live { _part: run(bar, part), _turn: turn }
}

/// A register as the driver last wrote it, or None while a write is landing.
/// The window is bytes and a model reads it a byte at a time, so a read that
/// straddles the driver's 32-bit store would mix old and new bytes, which no
/// real part sees. Two equal reads in a row cannot straddle the same store.
pub fn stable32(bar: &FakeBar, offset: usize) -> Option<u32> {
    let first = bar.wrote32(offset);
    (first == bar.wrote32(offset)).then_some(first)
}

/// How long `f` took, and what it returned. Timeout paths are held to have
/// waited at least their bound, not merely to have failed.
pub fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let out = f();
    (out, start.elapsed())
}
