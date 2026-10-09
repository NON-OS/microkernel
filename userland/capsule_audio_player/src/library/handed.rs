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

//! What Music does with the file the desktop shell handed it (the file
//! manager's Enter on an .mp3 or .wav). The shell answers "is there a file
//! for me" with a 4-byte status and then the path; a bare status means it
//! holds nothing for this app. Kept apart from the IPC so it can be proven.

use super::playable::is_playable;
use super::track::Track;

/// The handed-over file, as Music takes it.
#[derive(Debug, PartialEq, Eq)]
pub enum Handed<'a> {
    /// Nothing was held for this app.
    None,
    /// A track of the /audio library: that entry is selected and played.
    InLibrary(usize),
    /// A track from anywhere else: played as now playing, the library and
    /// its queue left as they are.
    Outside(&'a str),
    /// A file Music cannot decode, said on screen rather than opened.
    Refuse(&'a str),
}

/// `body` is the reply after the wire header; `tracks` the library.
pub fn handed<'a>(body: &'a [u8], tracks: &[Track]) -> Handed<'a> {
    let Some(path) = body.get(4..).and_then(|p| core::str::from_utf8(p).ok()) else {
        return Handed::None;
    };
    if !path.starts_with('/') {
        return Handed::None;
    }
    if !is_playable(path) {
        return Handed::Refuse(path);
    }
    match tracks.iter().position(|t| same_path(&t.path, path)) {
        Some(i) => Handed::InLibrary(i),
        None => Handed::Outside(path),
    }
}

/// Paths naming the same file, whatever doubled or trailing slashes they
/// were written with: the library's come from the store's own listing, a
/// handed path is whatever its sender built.
pub fn same_path(a: &str, b: &str) -> bool {
    a.split('/').filter(|s| !s.is_empty()).eq(b.split('/').filter(|s| !s.is_empty()))
}
