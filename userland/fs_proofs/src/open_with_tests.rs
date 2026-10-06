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

use crate::fm_logic::open_arg_reply::reply_dir;
use crate::fm_logic::open_with_table::handlers_for;
use crate::fm_logic::undo::{Op, UndoStack};

fn names(path: &str) -> Vec<&'static str> {
    handlers_for(path).iter().map(|h| h.name).collect()
}

#[test]
fn open_with_maps_known_extensions_to_app_names() {
    assert_eq!(names("/a/notes.txt"), vec!["Editor"]);
    assert_eq!(names("/a/song.mp3"), vec!["Music"]);
    assert_eq!(names("/a/clip.avi"), vec!["Video"]);
    assert_eq!(names("/a/photo.png"), vec!["Image Viewer"]);
    assert_eq!(handlers_for("/a/notes.txt")[0].service, "app.text_editor");
    assert_eq!(handlers_for("/a/photo.jpg")[0].service, "app.image_viewer");
}

#[test]
fn music_takes_mp3_and_wav_and_files_says_so() {
    // Music takes the handed-over path (capsule_audio_player's open_arg.rs),
    // so the line names it like the other apps rather than its library.
    for path in ["/home/nonos/song.mp3", "/audio/tone.wav", "/x/LOUD.MP3"] {
        let h = &handlers_for(path)[0];
        assert_eq!(h.service, "app.audio_player");
        assert_eq!(h.opened, b"opened in Music");
    }
}

#[test]
fn open_with_is_case_insensitive() {
    assert_eq!(names("/a/SONG.WAV"), vec!["Music"]);
}

#[test]
fn open_with_offers_no_app_that_cannot_read_the_file() {
    // Music decodes MP3 and WAV only, Video Motion-JPEG AVI only.
    assert!(handlers_for("/a/song.flac").is_empty());
    assert!(handlers_for("/a/film.mp4").is_empty());
    assert!(handlers_for("/a/film.mkv").is_empty());
}

#[test]
fn open_with_returns_empty_for_an_unclaimed_type() {
    assert!(handlers_for("/a/blob.zzz").is_empty());
    assert!(handlers_for("/a/noext").is_empty());
}

#[test]
fn every_handler_says_where_the_file_went() {
    for path in ["/a.txt", "/a.mp3", "/a.avi", "/a.png", "/a.html"] {
        for h in handlers_for(path) {
            assert!(h.service.starts_with("app."), "{}", h.service);
            assert!(!h.opened.is_empty() && !h.name.is_empty());
        }
    }
}

#[test]
fn the_shell_hands_over_a_folder_as_a_listing_prefix() {
    let mut body = vec![0u8; 4];
    assert_eq!(reply_dir(&body), None, "a bare status holds nothing");
    body.extend_from_slice(b"/home/nonos/Projects");
    assert_eq!(reply_dir(&body).as_deref(), Some("/home/nonos/Projects/"));
    let mut slash = vec![0u8; 4];
    slash.extend_from_slice(b"/downloads/");
    assert_eq!(reply_dir(&slash).as_deref(), Some("/downloads/"));
    let mut rel = vec![0u8; 4];
    rel.extend_from_slice(b"Projects");
    assert_eq!(reply_dir(&rel), None);
    assert_eq!(reply_dir(&[0, 0]), None);
}

#[test]
fn undo_pops_in_reverse_order() {
    let mut u = UndoStack::default();
    u.push(Op::Unlink { path: alloc::string::String::from("/a") });
    u.push(Op::Rmdir { path: alloc::string::String::from("/d") });
    assert!(matches!(u.pop(), Some(Op::Rmdir { .. })));
    assert!(matches!(u.pop(), Some(Op::Unlink { .. })));
    assert!(u.pop().is_none());
}

#[test]
fn undo_is_bounded_at_16() {
    let mut u = UndoStack::default();
    for i in 0..20 {
        u.push(Op::Unlink { path: alloc::format!("/f{i}") });
    }
    assert!(matches!(u.pop(), Some(Op::Unlink { ref path }) if path == "/f19"));
    for _ in 0..15 {
        assert!(u.pop().is_some());
    }
    assert!(u.pop().is_none());
}

#[test]
fn one_action_on_many_entries_is_one_undo() {
    let ops = alloc::vec![
        Op::Rename {
            from: alloc::string::String::from("/b/x"),
            to: alloc::string::String::from("/a/x")
        },
        Op::Unlink { path: alloc::string::String::from("/b/y") },
    ];
    let mut u = UndoStack::default();
    u.push(Op::group(ops).expect("two ops make an entry"));
    assert!(matches!(u.pop(), Some(Op::Batch(ref v)) if v.len() == 2));
    assert!(u.is_empty());
}

#[test]
fn a_single_op_stands_alone_and_none_is_nothing() {
    let one = alloc::vec![Op::Rmdir { path: alloc::string::String::from("/d (copy)") }];
    assert!(matches!(Op::group(one), Some(Op::Rmdir { .. })));
    assert!(Op::group(alloc::vec::Vec::new()).is_none());
}

#[test]
fn undo_clear_empties_the_stack() {
    let mut u = UndoStack::default();
    u.push(Op::Unlink { path: alloc::string::String::from("/a") });
    u.clear();
    assert!(u.is_empty());
}
