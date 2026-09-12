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

//! What the measurement itself costs.
//!
//! Two fenced counter reads are not free. On current hardware the pair runs to
//! tens of cycles, which is the same order as a bare syscall entry and exit:
//! timing one without subtracting the other reports the sum and calls it the
//! syscall. So the empty body is measured under exactly the same fences as a
//! real one, and the floor of that is taken off every sample.
//!
//! The floor rather than the median, because the cheapest observed pair is the
//! one least polluted by an interrupt landing mid-measurement. Subtracting a
//! median would over-subtract and flatter every result.

use super::tsc::read_serialised;

#[derive(Clone, Copy, Default)]
pub struct Overhead {
    pub cycles: u64,
}

impl Overhead {
    /// Calibrate against an empty body.
    pub fn measure(rounds: u32) -> Self {
        let mut floor = u64::MAX;
        for _ in 0..rounds {
            let a = read_serialised();
            let b = read_serialised();
            let d = b.saturating_sub(a);
            if d < floor {
                floor = d;
            }
        }
        Overhead { cycles: if floor == u64::MAX { 0 } else { floor } }
    }

    /// Take the overhead off one raw sample.
    ///
    /// Saturating: a sample that measured below the calibrated floor is noise
    /// at the resolution of the counter, and reporting it as zero is honest
    /// where wrapping to a huge number would not be.
    pub fn correct(&self, raw: u64) -> u64 {
        raw.saturating_sub(self.cycles)
    }
}
