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

//! Detailed timing descriptors, decoded alone: each honest one is taken
//! as written, each lie refused, and noise taken exactly when it scans.

use super::plan::Plan;
use super::rng::Rng;
use super::tally::run;
use super::widths::DTD;
use super::write::as_dtd;
use crate::base::dtd::Detailed;

#[test]
fn an_honest_descriptor_is_taken_as_written_and_every_lie_refused() {
    let n = run(0x6E0D_D7D0, &DTD, 90_000, |p, r| {
        Detailed::decode(&as_dtd(p, r)).map(|d| Plan::of(&d))
    });
    assert!(n.taken >= 9_000, "{}", n.taken);
    assert!(n.told.iter().all(|&k| k >= 2_000), "{:?}", n.told);
}

#[test]
fn noise_is_taken_exactly_when_it_could_scan() {
    let mut r = Rng::new(0x6E0D_0153);
    let (mut taken, mut refused) = (0usize, 0usize);
    for _ in 0..400_000 {
        let d = r.bytes::<18>();
        let Some(f) = Detailed::fields(&d) else { continue };
        let ours = Detailed::decode(&d);
        assert_eq!(ours, Plan::of(&f).scans().then_some(f), "{d:02X?}");
        match ours {
            Some(_) => taken += 1,
            None => refused += 1,
        }
    }
    assert!(taken >= 1_000 && refused >= 1_000, "{taken} {refused}");
}
