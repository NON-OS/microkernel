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

//! What the gen3 path needs beyond registers: DMA regions shared with the
//! device, and a clock for bounded waits. Broker grants and the uptime clock
//! back them in the capsule; plain memory and a modeled clock in the proofs.

/// One DMA region: its device-visible address and length, and bounds-checked
/// host access. Every access past the end is refused, never performed.
// A region is never empty: its length is the grant the driver mapped.
#[allow(clippy::len_without_is_empty)]
pub trait Region {
    fn len(&self) -> usize;
    fn dev(&self) -> u64;
    fn write(&self, off: usize, src: &[u8]) -> bool;
    fn read(&self, off: usize, dst: &mut [u8]) -> bool;
}

mod bytes;
mod spin;
pub use bytes::*;
pub use spin::spin_until;

/// Bounded waiting. `poll_for` re-checks `ready` until it holds or `ms`
/// milliseconds pass, and may sleep between checks; `spin_for` does the
/// same without ever yielding the CPU (`spin_until`); `delay_us` lets a
/// settle time elapse; `now_ms` is the time a deadline spanning many waits
/// is measured on.
pub trait Clock {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool;
    fn spin_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool;
    fn delay_us(&mut self, us: u32);
    fn now_ms(&mut self) -> u64;
}

/// The byte range `n` bytes at `off` cover in a region of `len` bytes, or
/// `None` when any of it falls outside (or the end overflows). A `Region`
/// backed by real memory checks every access with this before touching it.
pub fn span(len: usize, off: usize, n: usize) -> Option<core::ops::Range<usize>> {
    let end = off.checked_add(n)?;
    (end <= len).then_some(off..end)
}
