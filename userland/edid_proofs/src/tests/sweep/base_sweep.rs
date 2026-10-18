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

//! Base blocks whose descriptors, extension count and length each lie:
//! honest timings are all found, lies never, and the count is capped
//! by the blocks actually handed over.

use super::ext_did::did;
use super::gen::{slot, Kind};
use super::look::look;
use super::rng::Rng;
use crate::build::{base, seal};
use crate::edid::Edid;

#[test]
fn every_honest_slot_is_found_and_no_count_runs_past_the_bytes() {
    let mut r = Rng::new(0xBA5E_B10C);
    let (mut found, mut lied, mut refused, mut cut) = (0usize, 0usize, 0usize, 0usize);
    for _ in 0..30_000 {
        let mut b = base();
        if r.one_in(4) {
            r.fill(&mut b[8..54]);
        }
        let mut kinds = [Kind::Blank; 4];
        for (k, d) in kinds.iter_mut().zip(b[54..126].chunks_exact_mut(18)) {
            let s;
            (s, *k) = slot(&mut r);
            d.copy_from_slice(&s);
        }
        b[126] = r.byte();
        if !r.one_in(32) {
            seal(&mut b);
        }
        let mut raw = b.to_vec();
        for _ in 0..r.range(0, 3) {
            raw.extend_from_slice(&if r.one_in(2) { did(&mut r).bytes } else { r.bytes() });
        }
        if r.one_in(3) {
            raw.truncate(r.range(0, raw.len() as u32) as usize);
            cut += 1;
        }
        let Some(seen) = look(&raw) else {
            refused += 1;
            continue;
        };
        let count = Edid::parse(&raw).unwrap().extension_count();
        assert_eq!(count, (b[126] as usize).min(raw.len() / 128 - 1));
        let n = |k: Kind| kinds.iter().filter(|&&s| s == k).count();
        let (honest, noise) = (n(Kind::Honest), n(Kind::Noise));
        assert!((honest..=honest + noise).contains(&seen.base), "{kinds:?} {seen:?}");
        // Only the first slot that is not a display descriptor can be
        // the preferred timing, and only when it scans.
        let first = kinds.iter().find(|&&k| k != Kind::Display && k != Kind::Blank);
        let must = (first == Some(&Kind::Honest)) as usize;
        let may = must.max((first == Some(&Kind::Noise)) as usize);
        assert!((must..=may).contains(&seen.preferred_base), "{kinds:?} {seen:?}");
        found += honest;
        lied += n(Kind::Lied);
    }
    assert!(found >= 10_000 && lied >= 10_000, "{found} {lied}");
    assert!(refused >= 1_000 && cut >= 1_000, "{refused} {cut}");
}
