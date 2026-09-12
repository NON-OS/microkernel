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

/// Draw `paths`, which are relative to the tree's root and need not be sorted
/// by the caller beyond what the vfs returned.
///
/// Every intermediate directory is drawn whether or not it appears in the list
/// in its own right, because a listing that only names files still describes
/// the directories above them.
pub fn render(paths: &[Vec<u8>]) -> Vec<Row> {
    // Every distinct node, including the directories implied by longer paths.
    let mut nodes: Vec<Vec<&[u8]>> = Vec::new();
    for p in paths {
        let segs = segments(p);
        for depth in 1..=segs.len().min(MAX_DEPTH) {
            let node = segs[..depth].to_vec();
            if !nodes.contains(&node) {
                nodes.push(node);
            }
        }
    }
    nodes.sort();

    let mut out = Vec::new();
    // Which ancestor levels still have something below them to draw a line for.
    let mut open = [false; MAX_DEPTH + 1];
    for (i, node) in nodes.iter().enumerate() {
        let depth = node.len();
        let last = !nodes[i + 1..].iter().any(|n| shares_parent(node, n));
        if depth >= 1 {
            open[depth - 1] = !last;
        }

        let mut line = Vec::new();
        for level in open.iter().take(depth.saturating_sub(1)) {
            line.extend_from_slice(if *level { b"|   " } else { b"    " });
        }
        if depth > 0 {
            line.extend_from_slice(if last { b"`-- " } else { b"|-- " });
        }
        let name_at = line.len();
        line.extend_from_slice(node[depth - 1]);
        // A node is a directory when something in the listing sits under it.
        let is_dir = nodes.iter().any(|o| o.len() == depth + 1 && o[..depth] == node[..]);
        out.push(Row { line, name_at, is_dir });
    }
    out
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
