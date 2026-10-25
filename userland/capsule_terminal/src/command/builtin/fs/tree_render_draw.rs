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

//! The rows of a drawn tree, connectors and all, from the paths beneath its root.

use alloc::vec::Vec;

use super::{segments, shares_parent, Row, MAX_DEPTH};

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
