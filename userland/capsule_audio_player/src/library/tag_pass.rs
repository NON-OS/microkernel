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

//! Reading the library's tags a few files at a time, from the window's ticks.
//!
//! A tag sits at the start of the file (ID3v2) or in its last 128 bytes
//! (ID3v1), so each file costs two reads and no decoding. A library of a few
//! thousand songs still takes many seconds on a Celeron, and reading it at
//! start held the window that long: the list shows at once with file names,
//! and each row takes its title, artist and album as its tags are read.

extern crate alloc;

use alloc::string::String;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::{mk_getpid, mk_uptime_ms};

use super::tags::read_tags;
use super::{Library, Track};

/// The bytes read from a file's start: an ID3v2 tag's text frames come first
/// and fit well inside, and a cover that runs past is simply not shown.
const HEAD: u32 = 64 * 1024;
/// The ID3v1 block at the end.
const TAIL: u64 = 128;

impl Library {
    /// Whether some track's tags are still to be read.
    pub fn untagged(&self) -> bool {
        self.tracks.iter().any(|t| !t.tagged)
    }

    /// Read tags for as long as `budget_ms` allows. True when a row changed.
    pub fn tag_some(&mut self, budget_ms: i64) -> bool {
        let until = mk_uptime_ms() + budget_ms;
        let pid = mk_getpid();
        let mut changed = false;
        for t in self.tracks.iter_mut().filter(|t| !t.tagged) {
            changed |= tag_one(pid, t);
            if mk_uptime_ms() >= until {
                break;
            }
        }
        changed
    }
}

/// Read one track's tags into it. True when anything it shows changed. A
/// file that cannot be read keeps its file name and is not tried again.
fn tag_one(pid: u32, t: &mut Track) -> bool {
    t.tagged = true;
    let Ok(mut s) = VfsStream::open(pid, t.path.as_bytes()) else { return false };
    let Ok(head) = s.read_window(0, HEAD) else { return false };
    let size = vfs::stat(pid, t.path.as_bytes()).map_or(0, |(n, _)| n);
    let tail = if size > TAIL { s.read_window(size - TAIL, TAIL as u32).ok() } else { None };
    let tags = read_tags(&head, tail.as_deref());
    let mut changed = false;
    if let Some(title) = tags.title {
        changed |= set(&mut t.title, title);
    }
    let by = tags.artist.or(tags.album_artist);
    if let Some(by) = by {
        changed |= set(&mut t.artist, by.clone());
        t.by = by;
    }
    if let Some(album) = tags.album {
        changed |= set(&mut t.album, album);
    }
    t.number = tags.track;
    changed
}

fn set(field: &mut String, value: String) -> bool {
    if *field == value {
        return false;
    }
    *field = value;
    true
}
