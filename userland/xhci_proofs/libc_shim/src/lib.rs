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

//! What the included controller files take from `nonos_libc`.
//!
//! Standing in, not stubbing out: `Deadline` keeps real time so a wait that
//! the specification says must give up really gives up, `mk_yield` yields,
//! `mk_idle_ms` sleeps, and `mk_irq_wait` answers that there is no interrupt
//! grant, which is a state the driver handles by polling without one. Yields
//! and sleeps are counted per thread, so a test can tell a wait that parks
//! from one that spins. DMA grants are host memory (see [`dma`]).

mod dma;

use std::cell::Cell;
use std::time::{Duration, Instant};

pub use dma::{dma_host, mk_dma_map, mk_dma_unmap, DmaMapOut};

thread_local! {
    static YIELDS: Cell<u64> = const { Cell::new(0) };
    static SLEPT_MS: Cell<u64> = const { Cell::new(0) };
}

/// Yields made on this thread so far.
pub fn yields() -> u64 {
    YIELDS.with(|c| c.get())
}

/// Milliseconds asked of `mk_idle_ms` on this thread so far.
pub fn slept_ms() -> u64 {
    SLEPT_MS.with(|c| c.get())
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    SLEPT_MS.with(|c| c.set(c.get().saturating_add(ms)));
    std::thread::sleep(Duration::from_millis(ms));
    0
}

pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(timeout_ms) }
    }
    pub fn expired(&self) -> bool {
        Instant::now() >= self.end
    }
}

pub fn mk_yield() -> i64 {
    YIELDS.with(|c| c.set(c.get().saturating_add(1)));
    std::thread::yield_now();
    0
}

/// No interrupt path exists on the host. A negative answer is what the driver
/// gets from a kernel that refused the grant, and it copes the same way.
pub extern "C" fn mk_irq_wait(_grant: u64, _last_seq: u64, _timeout_ms: u64, _out: *mut u64) -> i64 {
    -1
}
