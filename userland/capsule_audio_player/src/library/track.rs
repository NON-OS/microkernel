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

extern crate alloc;
use alloc::string::{String, ToString};

pub struct Track {
    pub title: String,
    /// The line under the title: the artist its tags name, or the file's
    /// own name when they name none, never an artist made up.
    pub artist: String,
    pub format: String,
    pub path: String,
    pub dur_ms: u32,
    /// The artist its tags name; empty when they name none.
    pub by: String,
    /// The album its tags name; empty when they name none.
    pub album: String,
    /// Its number on that album.
    pub number: Option<u16>,
    /// Whether its tags have been read yet (`tag_pass.rs`), so each file is
    /// read once and the window stays live while a large library is.
    pub tagged: bool,
}

impl Track {
    pub fn from_path(path: &str) -> Self {
        let base = path.rsplit('/').next().unwrap_or(path);
        let (stem, ext) = split_ext(base);
        Track {
            title: prettify(stem),
            artist: base.to_string(),
            format: fmt_label(ext),
            path: path.to_string(),
            // Known once the track has loaded (`app.rs`); 0 until then.
            dur_ms: 0,
            by: String::new(),
            album: String::new(),
            number: None,
            tagged: false,
        }
    }
}

fn split_ext(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(i) if i > 0 => (&name[..i], &name[i + 1..]),
        _ => (name, ""),
    }
}

fn fmt_label(ext: &str) -> String {
    let mut s = String::new();
    for b in ext.bytes() {
        s.push(b.to_ascii_uppercase() as char);
    }
    if s.is_empty() {
        s.push_str("RAW");
    }
    s
}

fn prettify(stem: &str) -> String {
    let mut out = String::new();
    let mut cap = true;
    for b in stem.bytes() {
        if b == b'_' || b == b'-' || b == b' ' {
            out.push(' ');
            cap = true;
        } else if cap {
            out.push(b.to_ascii_uppercase() as char);
            cap = false;
        } else {
            out.push(b as char);
        }
    }
    if out.is_empty() {
        out.push_str("Untitled");
    }
    out
}
