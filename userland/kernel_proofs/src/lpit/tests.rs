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

//! Native C-states as firmware lays them out (recalled from Intel's LPIT
//! description): enabled and disabled states, with and without a counter.

use super::lpit_entry::{states, Counter, LpiState};

use super::build::{gas, lpit, native};

#[test]
fn two_states_with_their_counters() {
    let cpu = native(0, 0, 30_000, 3_000, &gas(0x7F, 64, 0x632), 0);
    let s0 = native(1, 0, 30_000, 3_000, &gas(0, 32, 0xFE00_013C), 10_000);
    let got = states(&lpit(&[cpu, s0]));
    let msr = Counter {
        space: 0x7F,
        bit_width: 64,
        bit_offset: 0,
        access_size: 4,
        address: 0x632,
        frequency: 0,
    };
    assert_eq!(got.len(), 2);
    let want = LpiState {
        id: 0,
        disabled: false,
        min_residency_us: 30_000,
        latency_us: 3_000,
        counter: Some(msr),
    };
    assert_eq!(got[0], want);
    assert_eq!(got[1].id, 1);
    assert_eq!(
        got[1].counter.map(|c| (c.space, c.address, c.frequency)),
        Some((0, 0xFE00_013C, 10_000))
    );
}

#[test]
fn disabled_and_counterless_states() {
    let off = native(2, 1, 5, 6, &gas(0x7F, 64, 0x632), 0);
    let no_counter = native(3, 2, 5, 6, &gas(0x7F, 64, 0x632), 0);
    let zero_address = native(4, 0, 5, 6, &gas(0, 32, 0), 0);
    let got = states(&lpit(&[off, no_counter, zero_address]));
    assert!(got[0].disabled && got[0].counter.is_some());
    assert!(!got[1].disabled && got[1].counter.is_none());
    assert!(got[2].counter.is_none());
}
