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

//! Time is kept in milliseconds, and a message says what ran out.

use super::fetch_fixtures::url_of;
use crate::browser::fetch::budget::budget;
use crate::browser::fetch::deadline::due;
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net::mixnet::Network;

pub fn reading(received: usize, progress_ms: i64) -> Fetch {
    let mut f = Fetch::new(url_of("http://10.0.2.2/"), 11, Phase::ReadBody, 0);
    f.received = received;
    f.progress_ms = progress_ms;
    f
}

#[test]
fn silence_runs_from_the_last_byte_not_the_start() {
    let b = budget(Network::Direct);
    assert_eq!(due(&reading(900, 11_000), b, 23_000), None, "12 s since the last byte");
    assert_eq!(due(&reading(900, 11_000), b, 23_001), Some("stalled"));
}

#[test]
fn a_steady_trickle_is_still_bounded() {
    let b = budget(Network::Direct);
    assert_eq!(due(&reading(5, 119_999), b, 120_000), None);
    assert_eq!(due(&reading(5, 119_999), b, 120_001), Some("stalled"));
}

#[test]
fn a_kept_connection_that_says_nothing_is_dead_sooner() {
    let mut f = reading(0, 0);
    f.keep_uses = 3;
    assert_eq!(due(&f, budget(Network::Direct), 4_000), None);
    assert_eq!(due(&f, budget(Network::Direct), 4_001), Some("kept connection dead"));
}

#[test]
fn the_mixnet_waits_in_proportion() {
    let (direct, mixnet) = (budget(Network::Direct), budget(Network::Nym));
    assert!(mixnet.silent_ms >= 10 * direct.silent_ms);
    assert!(mixnet.connect_ms > direct.connect_ms && mixnet.idle_ms > direct.idle_ms);
}
