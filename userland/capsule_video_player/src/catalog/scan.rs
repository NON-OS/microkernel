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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::list_paths;

use super::entry::is_playable;
use super::folders::ROOTS;
use super::media::MediaItem;
use super::probe::probe;
use super::says::{list_roots, scan_failure};

pub const MAX_ENTRIES: usize = 256;

/// The videos under the roots, and why none of the roots could be listed when
/// that is so: an unreadable store must not read as one with no videos.
pub fn scan(owner_pid: u32) -> (Vec<MediaItem>, Option<&'static str>) {
    let mut out: Vec<MediaItem> = Vec::new();
    let results: [_; ROOTS.len()] =
        list_roots(|i| collect(owner_pid, ROOTS[i].as_bytes(), &mut out));
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out.truncate(MAX_ENTRIES);
    // A probe the store did not answer ends the probing: the rest are
    // listed without their size, length and frame.
    for item in out.iter_mut() {
        if !probe(owner_pid, item) {
            break;
        }
    }
    (out, scan_failure(&results))
}

fn collect(owner_pid: u32, root: &[u8], out: &mut Vec<MediaItem>) -> Result<(), &'static str> {
    let paths = list_paths(owner_pid, root)?;
    for path in paths {
        // Only what the player decodes is listed (Motion-JPEG AVI). An .mp4,
        // .mkv or .mov would show in the library and then refuse to play.
        if !is_playable(&path) || out.iter().any(|m| m.path == path) {
            continue;
        }
        if let Some(item) = MediaItem::from_path(&path) {
            out.push(item);
        }
    }
    Ok(())
}
