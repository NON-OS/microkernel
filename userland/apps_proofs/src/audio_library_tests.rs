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

//! The audio player's library lists only files its decoder can open, and the
//! Artists and Albums tabs order tracks by what their tags name.

use crate::library::order::{by_album, by_artist};
use crate::library::playable::is_playable;
use crate::library::rescan_gap::{next_rescan_ms, SILENT_GAP_MS};
use crate::library::track::Track;

#[test]
fn the_library_lists_only_what_the_decoder_opens() {
    assert!(is_playable("/audio/song.mp3"));
    assert!(is_playable("/audio/Song.WAV"));
    assert!(is_playable("/audio/a.b.mp3"));
    // Listed before as audio though nothing decodes them.
    assert!(!is_playable("/audio/song.flac"));
    assert!(!is_playable("/audio/song.ogg"));
    assert!(!is_playable("/audio/notes.txt"));
    assert!(!is_playable("/audio/mp3"));
    assert!(!is_playable("/audio/.mp3"));
    assert!(!is_playable("/audio.mp3/readme"));
}

fn tagged(path: &str, by: &str, album: &str, number: Option<u16>, title: &str) -> Track {
    let mut t = Track::from_path(path);
    t.tagged = true;
    t.by = by.into();
    t.album = album.into();
    t.number = number;
    if !title.is_empty() {
        t.title = title.into();
    }
    t
}

fn library() -> [Track; 7] {
    [
        tagged("/m/z2.mp3", "zed", "Z", Some(2), "z two"),       // 0
        tagged("/m/loose.mp3", "", "Loose", None, "orphan"),     // 1
        tagged("/m/b.mp3", "Abba", "Gold", Some(2), "b"),        // 2
        tagged("/m/a.mp3", "abba", "gold", Some(1), "a"),        // 3
        tagged("/m/nonum.mp3", "Abba", "Gold", None, "c"),       // 4
        tagged("/m/single.mp3", "Abba", "", Some(1), "single"),  // 5
        Track::from_path("/m/untagged_file.mp3"),                // 6
    ]
}

#[test]
fn artists_hold_their_albums_together_in_track_order() {
    // Abba's Gold in number order, unnumbered last, then its untitled album;
    // then Zed; tracks naming no artist after every named one.
    assert_eq!(by_artist(&library()), vec![3, 2, 4, 5, 0, 1, 6]);
    assert!(by_artist(&[]).is_empty());
}

#[test]
fn albums_sort_by_name_then_artist() {
    let mut lib: Vec<Track> = library().into_iter().collect();
    lib.push(tagged("/m/hits1.mp3", "B", "Hits", Some(1), "x")); // 7
    lib.push(tagged("/m/hits2.mp3", "A", "Hits", Some(1), "y")); // 8
    assert_eq!(by_album(&lib), vec![3, 2, 4, 8, 7, 1, 0, 5, 6]);
}

#[test]
fn a_large_library_orders_quickly_and_whole() {
    let lib: Vec<Track> = (0..5000)
        .map(|i| {
            let by = format!("Artist {}", (i * 7919) % 250);
            let album = format!("Album {}", (i * 31) % 12);
            tagged(&format!("/m/{i}.mp3"), &by, &album, Some((i % 20) as u16 + 1), "")
        })
        .collect();
    let rows = by_artist(&lib);
    assert_eq!(rows.len(), 5000);
    let mut seen = rows.clone();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), 5000, "every track once");
    for w in rows.windows(2) {
        let (a, b) = (&lib[w[0]], &lib[w[1]]);
        assert!(a.by.to_lowercase() <= b.by.to_lowercase());
    }
}

/* An empty library is listed again from every paint and tick; a listing the
 * store did not answer cost five seconds each, 24 times over. */
#[test]
fn a_silent_listing_is_not_repeated_at_once() {
    assert_eq!(next_rescan_ms(Some("vfs ipc failed"), 1_000), 1_000 + SILENT_GAP_MS);
}

/* Longer than the five-second wait it saves. */
const _: () = assert!(SILENT_GAP_MS >= 5_000);

#[test]
fn an_answered_listing_may_be_followed_at_once() {
    assert_eq!(next_rescan_ms(None, 1_000), 1_000);
    assert_eq!(next_rescan_ms(Some("vfs list failed"), 1_000), 1_000);
}
