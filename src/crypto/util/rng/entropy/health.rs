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

//! SP 800-90B 4.4's health tests on every byte of the entropy capsule's,
//! pure, each at a false alarm rate of 2^-20 per sample. A source is
//! unproven until STARTUP samples pass, and again after any alarm.
//! kernel_proofs::entropy_health.

/// H, the min-entropy claimed per byte; the cutoffs follow from it.
pub(crate) const CLAIMED_BITS: u32 = 4;
/// Repetition count: 1 + ceil(20 / H) equal bytes in a row is an alarm.
pub(crate) const RCT_CUTOFF: u32 = 1 + 20u32.div_ceil(CLAIMED_BITS);
/// Adaptive proportion: 1 + CRITBINOM(512, 2^-H, 1 - 2^-20) of a window's first byte.
pub(crate) const APT_WINDOW: u32 = 512;
pub(crate) const APT_CUTOFF: u32 = 62;
pub(crate) const STARTUP: u32 = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fault {
    Repetition,
    Proportion,
}

pub(crate) struct Health {
    last: Option<u8>,
    run: u32,
    first: u8,
    count: u32,
    seen: u32,
    passed: u32,
}

impl Health {
    pub(crate) const fn new() -> Self {
        Self { last: None, run: 0, first: 0, count: 0, seen: 0, passed: 0 }
    }

    /// One sample through both tests. An alarm restarts the source.
    pub(crate) fn sample(&mut self, x: u8) -> Result<(), Fault> {
        self.run = if self.last == Some(x) { self.run + 1 } else { 1 };
        self.last = Some(x);
        if self.seen == 0 {
            (self.first, self.count) = (x, 0);
        }
        self.count += u32::from(x == self.first);
        self.seen = (self.seen + 1) % APT_WINDOW;
        let fault = match () {
            _ if self.run >= RCT_CUTOFF => Fault::Repetition,
            _ if self.count >= APT_CUTOFF => Fault::Proportion,
            _ => {
                self.passed = self.passed.saturating_add(1);
                return Ok(());
            }
        };
        *self = Self::new();
        Err(fault)
    }

    pub(crate) fn proven(&self) -> bool {
        self.passed >= STARTUP
    }
}
