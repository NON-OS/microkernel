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

//! The video library lists only what the player decodes, its search field
//! takes typed keys, and the Folders page filters by the folders the scan
//! actually reads.

use crate::catalog::entry::is_playable;
use crate::catalog::folders::{in_folder, LABELS, ROOTS};
use crate::catalog::media::MediaItem;
use crate::video_browse::{Browse, QUERY_MAX};
use crate::video_event::action::Action;
use crate::video_event::key::{from_key, from_library_key, KEY_BACKSPACE, KEY_ENTER, KEY_UP};

fn browse(paths: &[&str]) -> Browse {
    let mut b = Browse::new();
    b.items = paths.iter().filter_map(|p| MediaItem::from_path(p)).collect();
    b.reindex();
    b
}

fn titles(b: &Browse) -> Vec<&str> {
    (0..b.len()).filter_map(|i| b.get(i)).map(|m| m.title()).collect()
}

#[test]
fn only_motion_jpeg_avi_is_listed() {
    assert!(is_playable("/Movies/trip.avi"));
    assert!(is_playable("/Clips/A.AVI"));
    assert!(!is_playable("/Movies/trip.mp4"));
    assert!(!is_playable("/Movies/trip.mkv"));
    assert!(!is_playable("/Movies/trip.mov"));
    assert!(!is_playable("/Movies/.avi"));
}

#[test]
fn folders_match_the_scanned_roots() {
    assert_eq!(LABELS.len(), ROOTS.len());
    assert!(in_folder("/video.avi", 0));
    assert!(in_folder("/Movies/trip.avi", 1));
    assert!(!in_folder("/Movies/trip.avi", 0));
    assert!(!in_folder("/Movies/deep/trip.avi", 1));
    assert!(!in_folder("/Movies/trip.avi", ROOTS.len()));
}

#[test]
fn typing_filters_and_backspace_widens() {
    let mut b = browse(&["/Movies/alpha.avi", "/Movies/beta.avi", "/Clips/alps.avi"]);
    b.sel = 2;
    assert!(b.type_byte(b'a'));
    assert!(b.type_byte(b'L'));
    assert_eq!(b.query, "aL");
    assert_eq!(titles(&b), vec!["alpha", "alps"]);
    assert_eq!(b.sel, 0, "a new search starts from the top");
    assert!(b.type_byte(b'p'));
    assert!(b.type_byte(b'h'));
    assert_eq!(titles(&b), vec!["alpha"]);
    assert!(b.erase());
    assert!(b.erase());
    assert_eq!(titles(&b), vec!["alpha", "alps"]);
    assert!(b.erase() && b.erase());
    assert!(!b.erase(), "nothing left to erase");
    assert_eq!(b.len(), 3);
}

#[test]
fn the_search_takes_printable_ascii_up_to_its_length() {
    let mut b = browse(&["/a.avi"]);
    assert!(!b.type_byte(0x08));
    assert!(!b.type_byte(0x7F));
    for _ in 0..QUERY_MAX {
        assert!(b.type_byte(b'x'));
    }
    assert!(!b.type_byte(b'x'));
    assert_eq!(b.query.len(), QUERY_MAX);
}

#[test]
fn a_folder_shows_only_its_videos() {
    let mut b = browse(&["/top.avi", "/Movies/film.avi", "/Clips/clip.avi", "/Clips/more.avi"]);
    assert!(b.set_folder(Some(4)));
    assert_eq!(titles(&b), vec!["clip", "more"]);
    assert!(!b.set_folder(Some(4)), "the same folder again changes nothing");
    assert!(b.type_byte(b'm'));
    assert_eq!(titles(&b), vec!["more"]);
    assert!(b.set_folder(None));
    assert_eq!(titles(&b), vec!["film", "more"]);
    // An index past the roots means every folder.
    assert!(b.set_folder(Some(0)));
    assert!(b.set_folder(Some(ROOTS.len())));
    assert_eq!(b.folder, None);
}

#[test]
fn list_page_keys_type_into_the_search() {
    assert_eq!(from_library_key(b'q' as u32), Action::Type(b'q'));
    assert_eq!(from_library_key(b' ' as u32), Action::Type(b' '));
    assert_eq!(from_library_key(KEY_BACKSPACE), Action::Erase);
    assert_eq!(from_library_key(KEY_ENTER), Action::OpenSelected);
    assert_eq!(from_library_key(KEY_UP), Action::MoveSel(-1));
    // The player has no sound, so its keys set no volume and mute nothing.
    assert_eq!(from_key(KEY_UP), Action::None);
    assert_eq!(from_key(b'm' as u32), Action::None);
    assert_eq!(from_key(b' ' as u32), Action::TogglePlay);
}

#[test]
fn a_file_from_the_shell_plays_only_when_it_is_playable() {
    use crate::video_open_arg::{arg_of, Arg};
    let with = |path: &str| {
        let mut body = vec![0u8; 4];
        body.extend_from_slice(path.as_bytes());
        body
    };
    assert_eq!(arg_of(&[0, 0, 0, 0]), Arg::None, "a bare status holds no file");
    assert_eq!(arg_of(&[0, 0]), Arg::None);
    assert_eq!(arg_of(&with("/Movies/trip.avi")), Arg::Play("/Movies/trip.avi"));
    assert_eq!(arg_of(&with("/Movies/trip.mp4")), Arg::Refuse);
    assert_eq!(arg_of(&with("trip.avi")), Arg::None, "only an absolute path");
}

#[test]
fn where_a_video_was_left_is_where_it_opens() {
    let mut b = browse(&["/Movies/trip.avi", "/Clips/a.avi"]);
    let item = b.item_by_path_mut("/Movies/trip.avi").expect("listed");
    item.duration_ms = 60_000;
    assert_eq!(item.resume_point(), 0, "never played opens at the start");
    assert_eq!(item.permille(), 0);
    item.resume_ms = 30_000;
    assert_eq!(item.resume_point(), 30_000);
    assert_eq!(item.permille(), 500, "the card's watched bar is half full");
    assert!(b.item_by_path("/Clips/a.avi").is_some_and(|m| m.resume_ms == 0));
}

#[test]
fn a_video_left_at_its_end_opens_at_the_start() {
    let mut b = browse(&["/Movies/trip.avi"]);
    let item = b.item_by_path_mut("/Movies/trip.avi").expect("listed");
    item.duration_ms = 60_000;
    item.resume_ms = 60_000;
    assert_eq!(item.resume_point(), 0);
    assert_eq!(item.permille(), 1000, "watched through");
    item.resume_ms = 59_000;
    assert_eq!(item.resume_point(), 0, "a second from the end is the end");
}

#[test]
fn a_path_found_whatever_the_search_shows() {
    let mut b = browse(&["/Movies/trip.avi", "/Clips/a.avi"]);
    for &c in b"trip" {
        b.type_byte(c);
    }
    assert_eq!(b.len(), 1);
    assert!(b.item_by_path("/Clips/a.avi").is_some());
    assert!(b.item_by_path("/Clips/missing.avi").is_none());
}
