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

//! A file the desktop shell hands Music (the file manager's Enter on an .mp3
//! or .wav): an /audio track selects its library entry, a track from
//! elsewhere plays as now playing, and a file it cannot decode is refused.

use crate::library::handed::{handed, same_path, Handed};
use crate::library::track::Track;

/// The shell's reply after the wire header: a 4-byte status, then the path.
fn reply(path: &[u8]) -> Vec<u8> {
    let mut body = vec![0u8; 4];
    body.extend_from_slice(path);
    body
}

fn library() -> Vec<Track> {
    ["/audio/boot_tone.wav", "/audio/night_drive.mp3", "/audio/Morning.MP3"]
        .iter()
        .map(|p| Track::from_path(p))
        .collect()
}

#[test]
fn a_bare_status_holds_nothing() {
    let tracks = library();
    assert_eq!(handed(&[0, 0, 0, 0], &tracks), Handed::None);
    assert_eq!(handed(&[0, 0], &tracks), Handed::None);
    assert_eq!(handed(&[], &tracks), Handed::None);
}

#[test]
fn a_track_of_the_library_selects_its_entry() {
    let tracks = library();
    assert_eq!(handed(&reply(b"/audio/night_drive.mp3"), &tracks), Handed::InLibrary(1));
    assert_eq!(handed(&reply(b"/audio/boot_tone.wav"), &tracks), Handed::InLibrary(0));
    assert_eq!(handed(&reply(b"/audio/Morning.MP3"), &tracks), Handed::InLibrary(2));
}

#[test]
fn a_library_track_is_found_however_its_slashes_were_written() {
    let tracks = library();
    assert_eq!(handed(&reply(b"/audio//night_drive.mp3"), &tracks), Handed::InLibrary(1));
    assert!(same_path("/audio/a.mp3", "//audio/a.mp3"));
    assert!(!same_path("/audio/a.mp3", "/audio2/a.mp3"));
    assert!(!same_path("/audio/a.mp3", "/audio/sub/a.mp3"));
}

#[test]
fn a_track_from_elsewhere_plays_outside_the_library() {
    let tracks = library();
    let body = reply(b"/home/nonos/Music/song.mp3");
    assert_eq!(handed(&body, &tracks), Handed::Outside("/home/nonos/Music/song.mp3"));
    // Same name, another folder: not the library's file.
    let body = reply(b"/downloads/night_drive.mp3");
    assert_eq!(handed(&body, &tracks), Handed::Outside("/downloads/night_drive.mp3"));
    // The store's paths are case-sensitive: another name is another file.
    let body = reply(b"/audio/Night_Drive.mp3");
    assert_eq!(handed(&body, &tracks), Handed::Outside("/audio/Night_Drive.mp3"));
}

#[test]
fn an_outside_track_plays_with_an_empty_library() {
    let body = reply(b"/audio/new.wav");
    assert_eq!(handed(&body, &[]), Handed::Outside("/audio/new.wav"));
}

#[test]
fn a_file_music_cannot_decode_is_refused() {
    let tracks = library();
    assert_eq!(handed(&reply(b"/audio/song.flac"), &tracks), Handed::Refuse("/audio/song.flac"));
    assert_eq!(handed(&reply(b"/docs/notes.txt"), &tracks), Handed::Refuse("/docs/notes.txt"));
    assert_eq!(handed(&reply(b"/audio/noext"), &tracks), Handed::Refuse("/audio/noext"));
}

#[test]
fn a_path_that_is_not_an_absolute_text_path_is_not_taken() {
    let tracks = library();
    assert_eq!(handed(&reply(b"audio/night_drive.mp3"), &tracks), Handed::None);
    assert_eq!(handed(&reply(b"/audio/\xffbad.mp3"), &tracks), Handed::None);
}

#[test]
fn a_handed_over_track_is_named_from_its_file() {
    let t = Track::from_path("/home/nonos/Music/late_night-mix.mp3");
    assert_eq!(t.title, "Late Night Mix");
    assert_eq!(t.format, "MP3");
    assert_eq!(t.path, "/home/nonos/Music/late_night-mix.mp3");
}
