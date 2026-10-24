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

//! Read one EDID every way the crate offers and hold each answer to the
//! bounds: nothing past the bytes given, and no timing that cannot scan.

use super::super::hostile_tests::exercise;
use super::plan::Plan;
use crate::blocks::BLOCK;
use crate::edid::Edid;
use crate::source::Source;
use crate::timing::Timing;

/// The detailed timings one walk yielded, by where they came from.
#[derive(Debug, Default)]
pub struct Seen {
    pub base: usize,
    pub cta: usize,
    pub did: usize,
    pub preferred_base: usize,
}

/// None when the base block is refused.
pub fn look(raw: &[u8]) -> Option<Seen> {
    exercise(raw);
    let e = Edid::parse(raw).ok()?;
    let have = raw.len() / BLOCK;
    assert!(e.extension_count() < have, "{} of {have}", e.extension_count());
    assert_eq!(e.raw(), &raw[..(e.extension_count() + 1) * BLOCK]);
    assert!(e.extensions().count() <= e.extension_count());
    let mut seen = Seen::default();
    e.for_each_timing(&mut |t| {
        fit(&t);
        let base = t.detail.is_some() && t.source == Source::Detailed;
        seen.preferred_base += (base && t.preferred) as usize;
        match (t.detail, t.source) {
            (None, _) => {}
            (_, Source::Detailed) => seen.base += 1,
            (_, Source::CtaDetailed) => seen.cta += 1,
            (_, _) => seen.did += 1,
        }
    });
    assert!(e.preferred().is_none_or(|d| Plan::of(&d).scans()));
    Some(seen)
}

/// Every timing names a picture and a rate; a detailed one also scans
/// and agrees with its own detail.
fn fit(t: &Timing) {
    assert!(t.width > 0 && t.height > 0 && t.refresh_mhz > 0, "{t:?}");
    if let Some(d) = t.detail {
        assert!(Plan::of(&d).scans(), "{t:?}");
        assert_eq!((t.width, t.height), (d.width(), d.height()));
        assert_eq!((t.refresh_mhz, t.interlaced), (d.refresh_mhz(), d.interlaced));
    }
}
