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

//! How the engine code reaches the hardware. The capsule implements these
//! over its volatile mapping of BAR0 and its monotonic clock; the proofs
//! implement them over a model of the display engine, so the same code is
//! what runs on both.

/// 32-bit registers in GTTMMADR's register half, and 64-bit GGTT entries
/// in its upper half, each addressed by byte offset from the BAR's start.
pub trait Mmio {
    fn read(&mut self, offset: u32) -> u32;
    fn write(&mut self, offset: u32, value: u32);
    fn write64(&mut self, offset: u32, value: u64);
    fn read64(&mut self, offset: u32) -> u64;
}

/// A wait's end. The shipping one is a clock deadline, never a count of
/// polls, and a clock that cannot be read counts as expired.
pub trait Expiry {
    fn expired(&self) -> bool;
}

/// A monotonic microsecond clock; None when it cannot be read, which a
/// wait treats as its deadline passed.
pub trait Clock {
    fn now_us(&self) -> Option<u64>;
}

/// A deadline `us` from now on `clock`.
pub struct Until<'a> {
    clock: &'a dyn Clock,
    end: Option<u64>,
}

impl<'a> Until<'a> {
    pub fn new(clock: &'a dyn Clock, us: u64) -> Self {
        Until { clock, end: clock.now_us().map(|t| t.saturating_add(us)) }
    }
}

impl Expiry for Until<'_> {
    fn expired(&self) -> bool {
        match (self.end, self.clock.now_us()) {
            (Some(end), Some(now)) => now >= end,
            _ => true,
        }
    }
}

/// Read, change the bits in `mask` to `value`, write back.
pub fn rmw(m: &mut dyn Mmio, offset: u32, mask: u32, value: u32) {
    let old = m.read(offset);
    m.write(offset, (old & !mask) | (value & mask));
}

/// Bits `hi..=lo` of `v`, shifted down.
pub fn field(v: u32, hi: u32, lo: u32) -> u32 {
    (v >> lo) & (u32::MAX >> (31 - (hi - lo)))
}
