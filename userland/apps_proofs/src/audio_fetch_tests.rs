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

//! Music's download from the Search field: the name the file is kept under,
//! and an address pasted or typed into a field sized for a few words.

use std::collections::HashSet;
use std::string::String;

use crate::audio_fetch_name::{file_name, free_path, MUSIC_DIR};
use crate::audio_ui::search_key::{
    paste_query, room, search_key, Pasted, SearchKey, ADDRESS_MAX, QUERY_MAX,
};

#[test]
fn the_name_is_the_address_last_segment_with_the_found_extension() {
    assert_eq!(file_name("/music/Night%20Drive.mp3", "mp3"), "Night Drive.mp3");
    assert_eq!(file_name("/a/b/track.MP3?sig=abc&exp=1#t=2", "mp3"), "track.mp3");
    // What the check found wins over what the address claimed.
    assert_eq!(file_name("/song.wav", "mp3"), "song.mp3");
    assert_eq!(file_name("/get?id=7", "mp3"), "get.mp3");
}

#[test]
fn a_name_never_escapes_the_folder_or_hides() {
    assert_eq!(file_name("/x/..%2F..%2Fetc%2Fpasswd", "mp3"), "etc_passwd.mp3");
    assert_eq!(file_name("/.hidden.mp3", "mp3"), "hidden.mp3");
    assert_eq!(file_name("/", "mp3"), "download.mp3");
    assert_eq!(file_name("/%2E%2E", "wav"), "download.wav");
    let name = file_name("/a%00b%0Ac.mp3", "mp3");
    assert!(!name.contains('\0') && !name.contains('\n'), "{name}");
}

#[test]
fn a_long_name_is_cut_on_a_character() {
    let long = "\u{e9}".repeat(200);
    let name = file_name(&format!("/{long}.mp3"), "mp3");
    assert!(name.len() <= 80 + 4, "{}", name.len());
    assert!(name.ends_with(".mp3"));
}

#[test]
fn a_taken_name_is_numbered() {
    let taken: HashSet<String> =
        [format!("{MUSIC_DIR}/song.mp3"), format!("{MUSIC_DIR}/song (2).mp3")].into();
    assert_eq!(free_path("song.mp3", |p| taken.contains(p)), format!("{MUSIC_DIR}/song (3).mp3"));
    assert_eq!(free_path("other.mp3", |p| taken.contains(p)), format!("{MUSIC_DIR}/other.mp3"));
}

#[test]
fn a_pasted_address_gets_an_address_room() {
    let address = format!("https://files.example/{}/song.mp3?sig={}", "d".repeat(60), "s".repeat(300));
    assert!(address.len() > QUERY_MAX);
    let mut q = String::new();
    assert_eq!(paste_query(&mut q, &address), Pasted::Whole);
    assert_eq!(q, address);
    assert_eq!(room(&q), ADDRESS_MAX);
}

#[test]
fn pasted_words_keep_the_search_room() {
    let mut q = String::new();
    assert_eq!(paste_query(&mut q, &"w".repeat(QUERY_MAX + 10)), Pasted::Cut);
    assert_eq!(q.len(), QUERY_MAX);
    let mut q = String::new();
    assert_eq!(paste_query(&mut q, "a\tb\u{7}c"), Pasted::Whole);
    assert_eq!(q, "a bc", "a tab is a space; a control character is left out");
    assert_eq!(paste_query(&mut q, ""), Pasted::Empty);
}

#[test]
fn an_address_past_its_room_is_cut_and_says_so() {
    let mut q = String::new();
    let address = format!("https://h/{}", "x".repeat(ADDRESS_MAX));
    assert_eq!(paste_query(&mut q, &address), Pasted::Cut);
    assert_eq!(q.len(), ADDRESS_MAX);
}

#[test]
fn a_typed_address_runs_past_the_search_room() {
    let mut q = String::new();
    let address = format!("https://h/{}.mp3", "x".repeat(QUERY_MAX * 2));
    for c in address.bytes() {
        assert_eq!(search_key(&mut q, c as u32), SearchKey::Edited);
    }
    assert_eq!(q, address);
}
