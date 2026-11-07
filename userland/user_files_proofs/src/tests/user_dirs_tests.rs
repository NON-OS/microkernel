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

//! The user-dirs.dirs a new home is seeded with (capsule_linux
//! file/private/seed_rule.rs), read as GLib's g_get_user_special_dir reads
//! it: each `XDG_<NAME>_DIR="$HOME/<rest>"` line, `$HOME` the guest's
//! /root. Documents, Downloads, Music, Pictures and Videos land on the
//! broker's folders through the personality's own rule; the desktop, public
//! and templates folders are the home itself, which the XDG spec reads as
//! no folder.

use crate::linux::file::private::seed_rule::USER_DIRS_TEXT;
use crate::linux::file::user::place_rule::{broker_path, HOME};

/// Each line's name and path with `$HOME` expanded and a trailing `/` cut,
/// as GLib does.
fn read(text: &[u8]) -> Vec<(String, Vec<u8>)> {
    let text = std::str::from_utf8(text).unwrap();
    let mut out = Vec::new();
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let (key, value) = line.split_once('=').expect("a key and a value");
        let name =
            key.strip_prefix("XDG_").and_then(|k| k.strip_suffix("_DIR")).expect("XDG_*_DIR");
        let value = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')).expect("quoted");
        let rest = value.strip_prefix("$HOME").expect("under $HOME");
        let mut path = [HOME, rest.as_bytes()].concat();
        while path.len() > 1 && path.ends_with(b"/") {
            path.pop();
        }
        out.push((name.to_string(), path));
    }
    out
}

#[test]
fn the_seeded_folders_are_the_brokers_and_no_other() {
    let dirs = read(USER_DIRS_TEXT);
    let names: Vec<&str> = dirs.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        [
            "DESKTOP",
            "DOCUMENTS",
            "DOWNLOAD",
            "MUSIC",
            "PICTURES",
            "PUBLICSHARE",
            "TEMPLATES",
            "VIDEOS"
        ]
    );
    for (name, path) in &dirs {
        match name.as_str() {
            "DOCUMENTS" => assert_eq!(broker_path(path), Some(&b"Documents"[..])),
            "DOWNLOAD" => assert_eq!(broker_path(path), Some(&b"Downloads"[..])),
            "MUSIC" => assert_eq!(broker_path(path), Some(&b"Music"[..])),
            "PICTURES" => assert_eq!(broker_path(path), Some(&b"Pictures"[..])),
            "VIDEOS" => assert_eq!(broker_path(path), Some(&b"Videos"[..])),
            _ => {
                assert_eq!(path.as_slice(), HOME, "{name} names a folder that is not the person's")
            }
        }
    }
}
