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

use alloc::vec::Vec;

use crate::browser::css::GridTrack;

/* A grid item's claim on the tracks of one axis: first track, span, and
 * its (min, max) size there. */
pub(in super::super) type Claim = (usize, usize, i32, i32);

/* One track being sized: its base size, the growth limit it may reach
 * when there is room, its fraction (hundredths of fr; 0 when not
 * flexible), and whether it is auto (auto tracks stretch into space left
 * over when no track is flexible). */
#[derive(Clone, Copy)]
pub(in super::super) struct Track {
    pub base: i32,
    pub limit: i32,
    pub fr: i64,
    pub auto: bool,
}

/* Initial sizes for `track` in a grid `size` px long on that axis (None
 * when it is not definite: then a percentage or a fraction sizes by
 * content, as auto does), and (min, max) the widest span-1 items in it. */
fn track(track: GridTrack, size: Option<i32>, items: (i32, i32)) -> Track {
    let fixed = |v: i32| Track { base: v, limit: v, fr: 0, auto: false };
    match (track, size) {
        (GridTrack::Px(p), _) => fixed(p as i32),
        (GridTrack::Pct(p), Some(s)) => fixed((s as i64 * p as i64 / 10_000) as i32),
        (GridTrack::Fr(f), Some(_)) => Track { base: 0, limit: 0, fr: f as i64, auto: false },
        (GridTrack::MinContent, _) => fixed(items.0),
        (GridTrack::MaxContent, _) => fixed(items.1),
        _ => Track { base: items.0, limit: items.1.max(items.0), fr: 0, auto: true },
    }
}

/* The span-1 (min, max) sizes of the items in each of `m` tracks. */
pub(in super::super) fn per_track(claims: &[Claim], m: usize) -> Vec<(i32, i32)> {
    let mut v = alloc::vec![(0, 0); m];
    for &(c, _, mn, mx) in claims.iter().filter(|c| c.1 == 1 && c.0 < m) {
        v[c] = (v[c].0.max(mn), v[c].1.max(mx));
    }
    v
}

/* Tracks of the given kinds, each with its items' span-1 (min, max). */
pub(in super::super) fn tracks_for(
    kinds: impl Iterator<Item = GridTrack>,
    size: Option<i32>,
    items: &[(i32, i32)],
) -> Vec<Track> {
    kinds.zip(items).map(|(k, &it)| track(k, size, it)).collect()
}
