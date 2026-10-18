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

//! DisplayID type VII and type I records, decoded alone: the same bar
//! as a descriptor, after each record's own clock unit is applied.

use super::plan::Plan;
use super::rng::Rng;
use super::tally::run;
use super::widths::{Widths, TYPE7};
use super::write::as_record;
use crate::displayid::type7::decode;

fn sweep(seed: u64, unit_khz: u32) {
    let w = Widths { clock_step_khz: unit_khz, clock_max_khz: unit_khz << 24, ..TYPE7 };
    let n = run(seed, &w, 60_000, |p, r| {
        decode(&as_record(p, unit_khz, r), unit_khz).map(|(d, _)| Plan::of(&d))
    });
    assert!(n.taken >= 6_000, "{}", n.taken);
    // Type VII adds one to every width, so a zero can never be written.
    assert_eq!(n.told[..4], [0; 4]);
    assert!(n.told[4..].iter().all(|&k| k >= 2_000), "{:?}", n.told);
}

#[test]
fn a_type_vii_record_is_taken_as_written_and_every_lie_refused() {
    sweep(0x0D1D_0007, 1);
}

#[test]
fn a_type_i_record_is_taken_as_written_and_every_lie_refused() {
    sweep(0x0D1D_0001, 10);
}

#[test]
fn record_noise_is_taken_only_when_it_could_scan() {
    let mut r = Rng::new(0x0D1D_0153);
    let (mut taken, mut refused) = (0usize, 0usize);
    for round in 0..400_000u32 {
        let d = r.bytes::<20>();
        let unit = [1, 10][round as usize % 2];
        let ours = decode(&d, unit).map(|(t, _)| t);
        assert!(ours.is_none_or(|t| Plan::of(&t).scans()), "{d:02X?}");
        match ours {
            Some(_) => taken += 1,
            None => refused += 1,
        }
    }
    assert!(taken >= 1_000 && refused >= 1_000, "{taken} {refused}");
}
