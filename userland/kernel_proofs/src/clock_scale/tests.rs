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

use super::scale::ticks_to_ms;

const GHZ_3: u64 = 3_000_000_000;
const DAY_MS: u64 = 86_400_000;

/// The exact answer, worked in 128 bits by the test itself.
fn exact(ticks: u64, hz: u64) -> u128 {
    ticks as u128 * 1000 / hz as u128
}

#[test]
fn agrees_with_the_narrow_product_wherever_that_fits() {
    let rates = [1_000, 1_193_182, 25_000_000, 1_000_000_000, GHZ_3, 5_400_000_000];
    let counts = [0, 1, 999, 1_000, 123_456_789, 3_600 * GHZ_3, u64::MAX / 1000];
    for hz in rates {
        for ticks in counts {
            assert_eq!(ticks_to_ms(ticks, hz), ticks * 1000 / hz, "{ticks} ticks at {hz} Hz");
        }
    }
}

#[test]
fn keeps_counting_past_seventy_one_days_at_three_gigahertz() {
    // The first count whose product no longer fits in 64 bits.
    let edge = u64::MAX / 1000 + 1;
    assert_eq!(ticks_to_ms(edge, GHZ_3) as u128, exact(edge, GHZ_3));
    for days in [72, 365, 3_650] {
        let ticks = days * 86_400 * GHZ_3;
        assert_eq!(ticks_to_ms(ticks, GHZ_3), days * DAY_MS, "{days} days");
    }
}

#[test]
fn a_full_counter_is_a_finite_time() {
    for hz in [1_000, 1_000_000_000, GHZ_3, u64::MAX] {
        assert_eq!(ticks_to_ms(u64::MAX, hz) as u128, exact(u64::MAX, hz), "{hz} Hz");
    }
}

#[test]
fn a_slow_counter_saturates_instead_of_wrapping() {
    // Below 1 kHz a full counter is more milliseconds than 64 bits hold.
    assert_eq!(ticks_to_ms(u64::MAX, 1), u64::MAX);
    assert_eq!(ticks_to_ms(u64::MAX, 999), u64::MAX);
    assert_eq!(ticks_to_ms(u64::MAX / 1000, 1), u64::MAX / 1000 * 1000);
}

#[test]
fn no_rate_is_no_time() {
    assert_eq!(ticks_to_ms(0, 0), 0);
    assert_eq!(ticks_to_ms(u64::MAX, 0), 0);
}

#[test]
fn more_ticks_never_read_as_less_time() {
    let mut last = 0;
    let mut ticks = 1u64;
    while ticks < u64::MAX / 4 {
        let ms = ticks_to_ms(ticks, GHZ_3);
        assert!(ms >= last, "{ticks} ticks read {ms} after {last}");
        last = ms;
        ticks = ticks * 3 + 7;
    }
}
