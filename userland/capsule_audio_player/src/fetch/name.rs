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

//! What a downloaded file is called in the music folder. Pure, so the proofs
//! hold it.

extern crate alloc;

use alloc::format;
use alloc::string::String;

/// The folder downloads are kept in, beside the desktop's workspace and
/// documents: memory on a live boot, kept on an installed one.
pub const MUSIC_DIR: &str = "/home/nonos/music";

/// The longest name kept; vfs paths are at most 255 bytes.
const NAME_MAX: usize = 80;

/// A file name from the address's last path segment: its percent escapes
/// read, anything but letters, digits, space, '-', '_' and '.' made '_', no
/// leading dot, and the extension the audio check found (`ext`, "mp3" or
/// "wav") whatever the address said. "download" when nothing is left.
pub fn file_name(target: &str, ext: &str) -> String {
    let path = target.split(['?', '#']).next().unwrap_or("");
    let last = path.rsplit('/').next().unwrap_or("");
    let decoded = percent_decode(last);
    let mut stem: String = decoded
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.') { c } else { '_' })
        .collect();
    if let Some(dot) = stem.rfind('.') {
        let tail = &stem[dot + 1..];
        if tail.eq_ignore_ascii_case("mp3") || tail.eq_ignore_ascii_case("wav") {
            stem.truncate(dot);
        }
    }
    let stem = stem.trim_matches(|c: char| c == '.' || c == ' ' || c == '_');
    let stem = if stem.is_empty() { "download" } else { stem };
    let keep = stem.char_indices().nth(NAME_MAX).map_or(stem.len(), |(i, _)| i);
    format!("{}.{ext}", &stem[..keep])
}

/// `name` in the music folder, numbered past any of `taken`: song.mp3,
/// then song (2).mp3.
pub fn free_path(name: &str, taken: impl Fn(&str) -> bool) -> String {
    let first = format!("{MUSIC_DIR}/{name}");
    if !taken(&first) {
        return first;
    }
    let (stem, ext) = name.rsplit_once('.').unwrap_or((name, ""));
    for n in 2..1000 {
        let next = format!("{MUSIC_DIR}/{stem} ({n}).{ext}");
        if !taken(&next) {
            return next;
        }
    }
    format!("{MUSIC_DIR}/{stem} (1000).{ext}")
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = alloc::vec::Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            let hex = core::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok());
            if let Some(v) = hex {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
