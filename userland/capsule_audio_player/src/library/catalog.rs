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
use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::list_paths;
use nonos_libc::mk_getpid;

use super::playable::is_playable;
use super::track::Track;

/// The library is the person's own music: what they download and what they
/// copy in. Nothing ships with the system.
const MUSIC_DIR: &str = crate::fetch::name::MUSIC_DIR;

pub struct Library {
    pub tracks: Vec<Track>,
    /// Why the music folder could not be listed, if it could not: an empty library
    /// from a failed listing is not the same as an empty folder.
    pub error: Option<&'static str>,
}

impl Library {
    pub fn scan() -> Self {
        let mut tracks = Vec::new();
        let mut error = None;
        // Made if absent, so Files shows the folder to copy music into.
        let pid = mk_getpid();
        for dir in ["/home", "/home/nonos", MUSIC_DIR] {
            let _ = nonos_app_skeleton::clients::vfs::mkdir(pid, dir.as_bytes());
        }
        match list_paths(pid, MUSIC_DIR.as_bytes()) {
            Ok(paths) => {
                // The count is for the boot harness; an empty library is
                // listed again up to 24 times, a line each.
                #[cfg(feature = "nonos-audio-player-smoketest")]
                {
                    let m = alloc::format!("[AP] audio n={}\n", paths.len());
                    nonos_libc::mk_debug(m.as_ptr(), m.len());
                }
                for p in paths {
                    if is_playable(&p) {
                        tracks.push(Track::from_path(&p));
                    }
                }
            }
            Err(e) => {
                let head = b"[AP] audio list ERR: ";
                nonos_libc::mk_debug(head.as_ptr(), head.len());
                nonos_libc::mk_debug(e.as_ptr(), e.len());
                nonos_libc::mk_debug(b"\n".as_ptr(), 1);
                error = Some(e);
            }
        }
        Library { tracks, error }
    }

    pub fn get(&self, i: usize) -> Option<&Track> {
        self.tracks.get(i)
    }
}
