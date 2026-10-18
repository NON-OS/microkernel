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

//! CTA-861 extensions whose data blocks and descriptor offset lie: an
//! honest descriptor is found even after a lie, until the list ends.

use super::ext_cta::{cta, Cta};
use super::gen::Kind;
use super::look::look;
use super::rng::Rng;
use crate::build::base;
use crate::build_ext::with;
use crate::edid::Edid;

/// The honest descriptors before the first zero clock, those plus the
/// noise that may happen to scan, and whether an honest one came after
/// a lie the reader had to pass over.
fn expected(c: &Cta) -> (usize, usize, bool) {
    let listed = c.slots.iter().take_while(|&&k| k != Kind::Display && k != Kind::Blank);
    let (mut honest, mut noise, mut after_lie, mut lied) = (0, 0, false, false);
    for &k in listed {
        lied |= k == Kind::Lied;
        after_lie |= lied && k == Kind::Honest;
        honest += (k == Kind::Honest) as usize;
        noise += (k == Kind::Noise) as usize;
    }
    (honest, honest + noise, after_lie)
}

#[test]
fn every_honest_descriptor_is_found_past_lies_and_lying_lengths() {
    let mut r = Rng::new(0xC7A8_6100);
    let (mut found, mut past_lie, mut lying) = (0usize, 0usize, 0usize);
    for _ in 0..30_000 {
        let exts: Vec<Cta> = (0..r.range(1, 3)).map(|_| cta(&mut r)).collect();
        let raw = with(base(), &exts.iter().map(|c| c.bytes).collect::<Vec<_>>());
        let seen = look(&raw).expect("the base block is honest");
        // An HF-EEODB block may say fewer extensions than were sent.
        let read = Edid::parse(&raw).unwrap().extension_count();
        let (mut want, mut most, mut known) = (0, 0, true);
        for c in &exts[..read.min(exts.len())] {
            let (honest, upper, after_lie) = expected(c);
            (want, most) = (want + honest, most + upper);
            past_lie += after_lie as usize;
            lying += c.slots.is_empty() as usize;
            known &= !c.slots.is_empty();
        }
        // Past a zero clock is padding, never a timing.
        assert!(seen.cta >= want && (!known || seen.cta <= most), "{seen:?} {want} {most}");
        found += want;
    }
    assert!(found >= 10_000 && past_lie >= 1_000 && lying >= 1_000, "{found} {past_lie} {lying}");
}
