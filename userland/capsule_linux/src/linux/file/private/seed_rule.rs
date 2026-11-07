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

//! Seeding a new home, as a rule over any place to write it. No crate paths
//! here, so host proofs mount this file as it is.
//!
//! A seed is a list of files, each a path relative to the home and its
//! bytes. It is written once: a file the home already has is kept, so a
//! person's edit is never overwritten, and once every file has landed a mark
//! is written that stops any later seed. A seed cut short (a full home)
//! writes no mark, and the next start fills only what is still missing.

use alloc::vec::Vec;

/// The mark a whole seed leaves, relative to the home.
pub const MARK: &[u8] = b".config/nonos/seeded";
/// Bounds on a seed an image ships: files, bytes, and components per path.
pub const MOST_FILES: usize = 64;
pub const MOST_BYTES: usize = 1 << 20;
pub const MOST_DEPTH: usize = 6;

/// Where GLib and Qt read the person's folders, relative to the home.
pub const USER_DIRS: &[u8] = b".config/user-dirs.dirs";
/// The person's Documents, Downloads, Music, Pictures and Videos, as the
/// broker serves them under the home (file/user/place_rule.rs); the desktop,
/// public and templates folders `$HOME/`, which the XDG spec reads as none,
/// so no file chooser offers a folder that is not the person's.
pub const USER_DIRS_TEXT: &[u8] = b"XDG_DESKTOP_DIR=\"$HOME/\"
XDG_DOCUMENTS_DIR=\"$HOME/Documents\"
XDG_DOWNLOAD_DIR=\"$HOME/Downloads\"
XDG_MUSIC_DIR=\"$HOME/Music\"
XDG_PICTURES_DIR=\"$HOME/Pictures\"
XDG_PUBLICSHARE_DIR=\"$HOME/\"
XDG_TEMPLATES_DIR=\"$HOME/\"
XDG_VIDEOS_DIR=\"$HOME/Videos\"
";
/// xdg-user-dirs-update leaves the names alone while this matches the locale.
pub const USER_DIRS_LOCALE: &[u8] = b".config/user-dirs.locale";
pub const USER_DIRS_LOCALE_TEXT: &[u8] = b"C\n";

/// Why a home was not written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The home or its store has no room: what landed stays, no mark.
    Full,
    /// Anything else the store refused.
    Refused,
}

/// A home being seeded: paths are relative to it.
pub trait Home {
    fn has(&mut self, path: &[u8]) -> Result<bool, Fault>;
    /// Made if missing; one that is there already is no fault.
    fn mkdir(&mut self, path: &[u8]) -> Result<(), Fault>;
    fn write(&mut self, path: &[u8], bytes: &[u8]) -> Result<(), Fault>;
}

/// What a seed did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Planted {
    /// The mark was there: nothing was looked at.
    Before,
    /// Written this time, and kept because the home had them.
    Now { written: usize, kept: usize },
}

/// A path an INDEX may name: relative, `/` separated, no empty, `.` or `..`
/// component, no NUL, components of at most 255 bytes and at most
/// MOST_DEPTH of them.
pub fn path_ok(p: &[u8]) -> bool {
    let parts = p.split(|b| *b == b'/');
    !p.is_empty()
        && p.split(|b| *b == b'/').count() <= MOST_DEPTH
        && !p.contains(&0)
        && parts.into_iter().all(|c| !c.is_empty() && c != b"." && c != b".." && c.len() <= 255)
}

/// The paths an image's INDEX names: one a line, `#` lines and blank ends
/// skipped. None for a list past the bounds or naming a path `path_ok`
/// refuses: a seed is taken whole or not at all.
pub fn index(text: &[u8]) -> Option<Vec<&[u8]>> {
    let mut out = Vec::new();
    for line in text.split(|b| *b == b'\n') {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        if line.is_empty() || line.starts_with(b"#") {
            continue;
        }
        if !path_ok(line) || out.len() == MOST_FILES || out.contains(&line) {
            return None;
        }
        out.push(line);
    }
    Some(out)
}

/// Writes `files` into `home` once (see the module's words).
pub fn plant(home: &mut impl Home, files: &[(&[u8], &[u8])]) -> Result<Planted, Fault> {
    if home.has(MARK)? {
        return Ok(Planted::Before);
    }
    let (mut written, mut kept) = (0, 0);
    for (path, bytes) in files {
        if home.has(path)? {
            kept += 1;
            continue;
        }
        parents(home, path)?;
        home.write(path, bytes)?;
        written += 1;
    }
    parents(home, MARK)?;
    home.write(MARK, b"")?;
    Ok(Planted::Now { written, kept })
}

/// Every directory above `path`, made.
fn parents(home: &mut impl Home, path: &[u8]) -> Result<(), Fault> {
    for (i, b) in path.iter().enumerate() {
        if *b == b'/' {
            home.mkdir(&path[..i])?;
        }
    }
    Ok(())
}
