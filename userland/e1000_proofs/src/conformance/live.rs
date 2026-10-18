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

//! The part running against a window: it completes a reset on the next look
//! and counts the resets it has completed, so a test can see where in the
//! driver's sequence each one fell.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use nonos_devmodel::{run, FakeBar, LiveDevice};

use crate::constants::regs::REG_CTRL;
use crate::constants::status::CTRL_RST;

static TURN: Mutex<()> = Mutex::new(());

/// A running part and the turn it holds. The driver waits a fixed number of
/// spins for the part to answer, and on a loaded runner a model thread that
/// shares the machine with a dozen other tests can be preempted for longer
/// than that budget. One live test at a time keeps the part scheduled.
pub struct Live {
    pub resets: Arc<AtomicU32>,
    _part: LiveDevice,
    _turn: MutexGuard<'static, ()>,
}

impl Live {
    pub fn resets(&self) -> u32 {
        self.resets.load(Ordering::SeqCst)
    }
}

/// The resetting part, running, proven to answer, and alone on the machine.
/// The reset that proves it is not counted.
pub fn live_part(bar: &Arc<FakeBar>) -> Live {
    live_with(bar, |_| {})
}

/// The same part doing `also` on every look as well.
pub fn live_with(bar: &Arc<FakeBar>, also: impl Fn(&FakeBar) + Send + 'static) -> Live {
    let turn = TURN.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let resets = Arc::new(AtomicU32::new(0));
    let n = Arc::clone(&resets);
    let part = run(bar, move |b| {
        let ctrl = b.wrote32(REG_CTRL);
        if ctrl & CTRL_RST != 0 {
            // Counted before it shows, so a driver that saw the reset end
            // never reads a count that misses it.
            n.fetch_add(1, Ordering::SeqCst);
            b.present32(REG_CTRL, ctrl & !CTRL_RST);
        }
        also(b);
    });
    bar.present32(REG_CTRL, CTRL_RST);
    while bar.wrote32(REG_CTRL) & CTRL_RST != 0 {
        std::thread::yield_now();
    }
    resets.store(0, Ordering::SeqCst);
    Live { resets, _part: part, _turn: turn }
}
