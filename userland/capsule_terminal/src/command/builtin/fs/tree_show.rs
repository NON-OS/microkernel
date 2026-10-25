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

//! A tree drawn into the scrollback: the rows, cut off at what it can spare, then the count.

use alloc::vec::Vec;

use super::super::tree_render::{counts, render};
use super::tree_summary::summary;
use crate::command::output::Output;

/// Most nodes drawn before stopping.
///
/// Deliberately well under `SCROLLBACK_ROWS`, because the scrollback is a ring
/// and a tree allowed to fill it would push out everything the reader ran
/// before, including the command they are reading the tree to answer.
const MAX_NODES: usize = 180;

/// The paths under the root, relative to it, drawn as rows beneath it.
pub(super) fn show(out: &mut Output<'_>, rel: Vec<Vec<u8>>) {
    let lines = render(&rel);
    let shown = lines.len().min(MAX_NODES);
    for row in lines.iter().take(shown) {
        // Directories take the same colour they take in a listing, so the two
        // commands agree about what a directory looks like.
        if row.is_dir {
            out.writeln_dir(&row.line[..row.name_at], &row.line[row.name_at..]);
        } else {
            out.writeln(&row.line);
        }
    }
    if lines.len() > shown {
        out.writeln(b"... more, narrow the path to see the rest");
    }

    let (dirs, files) = counts(&lines);
    out.writeln(&summary(dirs, files));
}
