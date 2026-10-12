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

//! How often an endpoint is worth asking for a report: as often as the
//! controller serves it, by the controller driver's own Interval rule, for
//! every speed and every bInterval a device can declare.

use super::period::period_ms;
use crate::xhci::interval::interrupt_interval;

#[test]
fn an_endpoint_is_asked_as_often_as_the_controller_serves_it() {
    for speed in 1..=5u8 {
        for b in 0..=255u8 {
            let exp = interrupt_interval(speed, speed >= 4, b);
            let served = ((125u64 << exp) / 1000).max(1);
            assert_eq!(period_ms(speed, b), served, "speed {speed}, bInterval {b}");
        }
    }
}

#[test]
fn common_endpoints_are_asked_at_their_own_rate() {
    // (speed, bInterval, ms): full and low speed keyboards of 10 ms frames
    // are served every 8, a high speed mouse of 4 every 1; the slowest of
    // each kind are held to 128 ms and 4096 ms.
    let named = [(1, 10, 8), (2, 10, 8), (1, 1, 1), (1, 0, 1), (1, 255, 128)];
    let fast = [(3, 4, 1), (3, 1, 1), (3, 7, 8), (4, 16, 4096), (5, 255, 4096)];
    for (speed, b, ms) in named.into_iter().chain(fast) {
        assert_eq!(period_ms(speed, b), ms, "speed {speed}, bInterval {b}");
    }
}
