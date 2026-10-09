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

//! The retransmission timeout the RTT samples give (RFC 6298 2.2, 2.3).

use crate::tcp::rtt::Rtt;
use crate::tcp::{RTO_INIT_MS, RTO_MIN_MS};

fn after(samples: &[u32]) -> u32 {
    let mut rtt = Rtt::new();
    for r in samples {
        rtt.on_sample(*r);
    }
    rtt.rto_ms()
}

#[test]
fn before_any_sample_the_timeout_is_the_initial_one() {
    assert_eq!(after(&[]), RTO_INIT_MS);
}

#[test]
fn the_first_sample_sets_the_mean_and_half_of_it_as_the_variation() {
    // SRTT 1000, RTTVAR 500, RTO = 1000 + 4 * 500.
    assert_eq!(after(&[1000]), 3000);
}

/*
 * The variation moves by how far a sample lies from the mean, whichever
 * side it lies on: 400 below and 400 above both take RTTVAR from 500 to
 * (3 * 500 + 400) / 4 = 475, and only SRTT differs (950 against 1050).
 */
#[test]
fn a_sample_below_the_mean_and_one_as_far_above_vary_it_alike() {
    assert_eq!(after(&[1000, 600]), 950 + 4 * 475);
    assert_eq!(after(&[1000, 1400]), 1050 + 4 * 475);
}

#[test]
fn a_tiny_sample_is_held_to_the_floor() {
    assert_eq!(after(&[1]), RTO_MIN_MS);
}
