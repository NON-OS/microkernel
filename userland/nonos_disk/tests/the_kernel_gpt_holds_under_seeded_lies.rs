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

//! The kernel's GPT reader over seeded disks, built sound then told one
//! lie: it believes exactly what common/gpt_sweep/model.rs believes, a lie
//! costing one entry costs no other, and what it names is always inside the
//! usable range with no two partitions sharing a sector.

extern crate alloc;

#[path = "common/gpt_sweep/mod.rs"]
mod gpt_sweep;
#[path = "common/kernel_gpt/mod.rs"]
#[allow(dead_code, unused_imports)]
mod kernel_gpt;

use gpt_sweep::build::disk;
use gpt_sweep::lie::{tell, Lie, LIES};
use gpt_sweep::rng::{rounds, Rng};
use gpt_sweep::view::{held, kernel, model};

#[test]
fn every_sound_disk_is_read_as_built() {
    let mut sizes = [0u32; 41];
    for seed in 1..rounds(3_000) {
        let d = disk(&mut Rng::new(seed));
        let got = kernel(&d).unwrap_or_else(|| panic!("seed {seed}: a sound disk refused"));
        assert_eq!(Some(got), model(&d), "seed {seed}");
        sizes[d.sectors.ilog2() as usize] += 1;
    }
    let spread = sizes.iter().filter(|&&n| n >= 30).count();
    assert!(cfg!(miri) || spread >= 20, "disk sizes over only {spread} powers of two");
}

#[test]
fn after_any_lie_the_reader_believes_what_the_rule_believes() {
    let skips_one = [Lie::PastLast, Lie::EndBeforeStart, Lie::BelowFirst];
    let mut told = [0u32; LIES.len()];
    for seed in 1..rounds(10_000) {
        let mut r = Rng::new(seed);
        let built = disk(&mut r);
        let sound = model(&built).unwrap();
        let (lie, d) = tell(&mut r, &built);
        let got = kernel(&d);
        assert_eq!(got, model(&d), "seed {seed}: {lie:?}");
        match got {
            Some(ref kept) if skips_one.contains(&lie) => {
                let why = format!("seed {seed}: {lie:?} cost more than its entry");
                assert_eq!(kept.len() + 1, sound.len(), "{why}");
                assert!(kept.iter().all(|p| sound.contains(p)), "seed {seed}: {lie:?}");
            }
            Some(_) => panic!("seed {seed}: {lie:?} was believed"),
            None => assert!(!skips_one.contains(&lie), "seed {seed}: {lie:?} cost the whole table"),
        }
        held(&d, got.as_deref(), seed);
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 300, "{lie:?} was told only {n} times");
    }
}
