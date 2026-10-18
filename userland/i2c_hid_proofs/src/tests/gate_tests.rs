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

//! The gate every call to driver.i2c_pci0 goes through: a controller driver
//! that does not answer is asked less and less often, not every few
//! milliseconds, and the first answer puts things back as they were.

use crate::i2c_client::gate::rule::{pause_ms, FIRST_PAUSE_MS, MAX_PAUSE_MS, UNANSWERED_LIMIT};
use crate::i2c_client::gate::Gate;

const TIMED_OUT: i64 = -110;

#[test]
fn a_few_unanswered_calls_close_the_gate_for_a_second() {
    let mut g = Gate::default();
    for _ in 0..UNANSWERED_LIMIT - 1 {
        g.record(TIMED_OUT, 100);
        assert!(g.open(100), "closed before the limit");
    }
    g.record(TIMED_OUT, 100);
    assert!(!g.open(100));
    assert!(!g.open(100 + FIRST_PAUSE_MS - 1));
    assert!(g.open(100 + FIRST_PAUSE_MS));
}

#[test]
fn each_pause_doubles_up_to_thirty_seconds() {
    assert_eq!(pause_ms(0), 1_000);
    assert_eq!(pause_ms(1), 2_000);
    assert_eq!(pause_ms(4), 16_000);
    assert_eq!(pause_ms(5), MAX_PAUSE_MS);
    assert_eq!(pause_ms(u32::MAX), MAX_PAUSE_MS);
    let mut g = Gate::default();
    let mut now = 0u64;
    let mut waits = Vec::new();
    for _ in 0..8 {
        for _ in 0..UNANSWERED_LIMIT {
            g.record(TIMED_OUT, now);
        }
        waits.push(g.closed_until_ms - now);
        now = g.closed_until_ms;
    }
    assert_eq!(waits, [1_000, 2_000, 4_000, 8_000, 16_000, 30_000, 30_000, 30_000]);
}
