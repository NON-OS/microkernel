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

//! Training against a sink whose AUX replies and link status lie: it ends
//! within its try counts and its time, a link it reports is one the sink
//! and the source both take, and a failure leaves the port down.

use super::profile::{profile, Profile, Status};
use super::rng::Rng;
use super::train_rig::run;

fn dpcd(r: &mut Rng) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[0] = [0x11, 0x12, 0x14, r.byte()][r.below(4) as usize];
    b[1] = [0x06, 0x0A, 0x14, 0x1E][r.below(4) as usize];
    b[2] = [1, 2, 4][r.below(3) as usize] | (r.byte() & 0xC0);
    b[0x0E] = r.below(5) as u8;
    b
}

#[test]
fn every_training_ends_bounded_and_leaves_the_port_as_it_says() {
    let mut r = Rng::new(0x7A1_7E57);
    let mut seen = [0usize; 4];
    for i in 0..20_000u64 {
        let (p, b) = (profile(&mut r), dpcd(&mut r));
        let max = (
            [1620, 2700, 3000, 5400, 8100][r.below(5) as usize],
            [1, 2, 3, 4][r.below(4) as usize],
        );
        seen[run(i, p, b, max)] += 1;
    }
    assert!(seen.iter().all(|&n| n >= 500), "[trained, failed, aux, transmitter] {seen:?}");
}

#[test]
fn a_sink_that_flips_its_ask_or_never_locks_fails_within_the_passes() {
    let clean = |status| Profile {
        timeout: 0,
        defer: 0,
        nack: 0,
        undefined: 0,
        empty: 0,
        odd_len: 0,
        status,
        start_fails: false,
    };
    let b = [0x14, 0x1E, 0x84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for status in [Status::Flips, Status::Never] {
        assert_eq!(run(1, clean(status), b, (8100, 4)), 1, "{status:?}");
    }
    assert_eq!(run(1, clean(Status::Settles(3)), b, (8100, 4)), 0, "an honest sink trains");
}
