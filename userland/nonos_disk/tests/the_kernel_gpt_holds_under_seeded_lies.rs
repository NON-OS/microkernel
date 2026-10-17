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

//! The kernel's GPT reader over seeded disks with both headers, built
//! sound then told one lie in either: it names exactly what the rule in
//! common/gpt_sweep/rule.rs names, takes the backup when only the primary
//! lies, says when a holding backup disagrees, and whatever it names lies
//! in the usable range with no sector shared.

extern crate alloc;

#[path = "common/gpt_sweep/mod.rs"]
mod gpt_sweep;
#[path = "common/kernel_gpt/mod.rs"]
#[allow(dead_code, unused_imports)]
mod kernel_gpt;

use gpt_sweep::build::disk;
use gpt_sweep::expect::expected;
use gpt_sweep::lie::{tell, LIES};
use gpt_sweep::rng::{rounds, Rng};
use gpt_sweep::rule::{rule, Named};
use gpt_sweep::view::{held, kernel};

#[test]
fn every_sound_disk_is_read_from_its_primary_as_built() {
    let mut sizes = [0u32; 41];
    for seed in 1..rounds(3_000) {
        let d = disk(&mut Rng::new(seed));
        let Named::Primary(parts) = kernel(&d) else {
            panic!("seed {seed}: a sound disk not read")
        };
        assert_eq!(Named::Primary(parts), rule(&d), "seed {seed}");
        sizes[d.sectors.ilog2() as usize] += 1;
    }
    let spread = sizes.iter().filter(|&&n| n >= 30).count();
    assert!(cfg!(miri) || spread >= 20, "disk sizes over only {spread} powers of two");
}

#[test]
fn after_any_lie_in_either_header_the_reader_names_what_the_rule_names() {
    let mut told = [[0u32; 2]; LIES.len()];
    for seed in 1..rounds(20_000) {
        let mut r = Rng::new(seed);
        let built = disk(&mut r);
        let Named::Primary(sound) = rule(&built) else { panic!("seed {seed}: built unsound") };
        let (lie, w, broke, d) = tell(&mut r, &built);
        let got = kernel(&d);
        assert_eq!(got, rule(&d), "seed {seed}: {lie:?} in header {w}");
        let why = format!("seed {seed}: {lie:?} in header {w}, primary broken {broke}");
        assert_eq!(got, expected(lie, w, broke, &sound, &got), "{why}");
        held(&d, &got, seed);
        told[LIES.iter().position(|l| *l == lie).unwrap()][w] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || n[0] + n[1] >= 300, "{lie:?} was told only {n:?} times");
    }
}
