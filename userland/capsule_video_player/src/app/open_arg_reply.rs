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

//! What the player does with the path in the shell's answer to "is there a
//! file for me to open". The shell answers with a 4-byte status and then the
//! path; a bare status means it holds nothing for this app. Kept apart from
//! the IPC so it can be proven.

use crate::catalog::entry::is_playable;

/// The handed-over file, as the player takes it.
#[derive(Debug, PartialEq, Eq)]
pub enum Arg<'a> {
    /// Nothing was held for this app.
    None,
    /// A video it plays.
    Play(&'a str),
    /// A file it cannot play, said in the status line rather than opened.
    Refuse,
}

/// `body` is the reply after the wire header.
pub fn arg_of(body: &[u8]) -> Arg<'_> {
    let Some(path) = body.get(4..).and_then(|p| core::str::from_utf8(p).ok()) else {
        return Arg::None;
    };
    if !path.starts_with('/') {
        return Arg::None;
    }
    if is_playable(path) {
        Arg::Play(path)
    } else {
        Arg::Refuse
    }
}
