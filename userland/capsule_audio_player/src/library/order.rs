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

//! The Library's Artists and Albums orders: track indices sorted so a
//! person browsing finds an artist's albums together and an album's tracks
//! in their order. Names are compared without case, and a track whose tags
//! name no artist or album sorts after every named one.

extern crate alloc;

use alloc::vec::Vec;
use core::cmp::Ordering;

use super::track::Track;

/// By artist, then album, then the track's number, then title.
pub fn by_artist(tracks: &[Track]) -> Vec<usize> {
    let mut rows: Vec<usize> = (0..tracks.len()).collect();
    rows.sort_by(|&a, &b| {
        let (ta, tb) = (&tracks[a], &tracks[b]);
        named(artist(ta), artist(tb))
            .then_with(|| named(&ta.album, &tb.album))
            .then_with(|| in_album(ta, tb))
    });
    rows
}

/// By album, then artist (two artists' albums of one name stay apart), then
/// the track's number, then title.
pub fn by_album(tracks: &[Track]) -> Vec<usize> {
    let mut rows: Vec<usize> = (0..tracks.len()).collect();
    rows.sort_by(|&a, &b| {
        let (ta, tb) = (&tracks[a], &tracks[b]);
        named(&ta.album, &tb.album)
            .then_with(|| named(artist(ta), artist(tb)))
            .then_with(|| in_album(ta, tb))
    });
    rows
}

fn artist(t: &Track) -> &str {
    &t.by
}

fn in_album(a: &Track, b: &Track) -> Ordering {
    let num = |t: &Track| t.number.unwrap_or(u16::MAX);
    num(a).cmp(&num(b)).then_with(|| folded(&a.title, &b.title))
}

/// Without case; an empty name after every other.
fn named(a: &str, b: &str) -> Ordering {
    match (a.is_empty(), b.is_empty()) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => folded(a, b),
    }
}

fn folded(a: &str, b: &str) -> Ordering {
    let a = a.bytes().map(|c| c.to_ascii_lowercase());
    a.cmp(b.bytes().map(|c| c.to_ascii_lowercase()))
}
