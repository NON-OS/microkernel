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

//! The bounds a wall-clock correction must fall in, and its offset, pure.
//! kernel_proofs::time_offset.

const CLOCK_FLOOR_MS: u64 = 1_735_689_600_000;
const CLOCK_CEIL_MS: u64 = 4_102_444_800_000;

pub fn clamp_ok(correct_ms: u64) -> bool {
    correct_ms >= CLOCK_FLOOR_MS && correct_ms <= CLOCK_CEIL_MS
}

pub fn compute_offset_ms(correct_ms: u64, base_ms: u64) -> i64 {
    correct_ms as i64 - base_ms as i64
}

#[cfg(test)]
mod tests {
    use super::{clamp_ok, compute_offset_ms};

    #[test]
    fn rejects_prehistoric_and_future() {
        assert!(!clamp_ok(0));
        assert!(!clamp_ok(1_600_000_000_000));
        assert!(!clamp_ok(5_000_000_000_000));
        assert!(clamp_ok(1_800_000_000_000));
    }

    #[test]
    fn offset_is_correct_minus_base() {
        assert_eq!(compute_offset_ms(1_800_000_010_000, 1_800_000_000_000), 10_000);
        assert_eq!(compute_offset_ms(1_800_000_000_000, 1_800_000_010_000), -10_000);
    }
}
