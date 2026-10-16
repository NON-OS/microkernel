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

//! Turning a flat list of paths into a drawn tree.
//!
//! The vfs answers a prefix with every path beneath it, so the shape is
//! already there and only has to be read out. Kept apart from the command that
//! fetches it, and free of any call into the system, so the drawing can be
//! checked against awkward input on a host rather than only by looking at it.
//!
//! A branch continues under a node when that node has a later sibling, which
//! is what the trailing `|` columns mean. Getting that wrong is the classic
//! way a drawn tree ends up with lines that lead nowhere.

use alloc::vec::Vec;

#[path = "tree_render_draw.rs"]
mod tree_render_draw;

pub use tree_render_draw::render;

/// Longest branch drawn. A tree deep enough to reach this is a tree nobody is
/// reading, and the indent alone would fill the width.
pub const MAX_DEPTH: usize = 12;

/// Split a path into its segments, dropping empties so a trailing or doubled
/// slash cannot invent a level.
fn segments(path: &[u8]) -> Vec<&[u8]> {
    path.split(|&b| b == b'/').filter(|s| !s.is_empty()).collect()
}

/// Whether `later` sits under the same parent as `here`, at the same depth.
fn shares_parent(here: &[&[u8]], later: &[&[u8]]) -> bool {
    later.len() == here.len() && later[..here.len() - 1] == here[..here.len() - 1]
}

/// One drawn row: the whole line, where the name starts within it, and
/// whether that name is a directory.
///
/// The parts are kept apart so the caller can colour the name and leave the
/// connectors alone, which is what `ls` already does for a listing row. A
/// renderer that returned only finished bytes would force the caller to find
/// the name again by counting connector characters.
pub struct Row {
    pub line: Vec<u8>,
    pub name_at: usize,
    pub is_dir: bool,
}

/// How many directories and how many leaves were drawn.
///
/// Counted from the drawn rows rather than worked out again from the paths, so
/// the summary can never disagree with the picture above it about what is a
/// directory.
pub fn counts(rows: &[Row]) -> (usize, usize) {
    let dirs = rows.iter().filter(|r| r.is_dir).count();
    (dirs, rows.len() - dirs)
}
