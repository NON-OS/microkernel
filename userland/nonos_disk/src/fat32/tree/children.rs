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

//! The recursive step of placement: a directory's children, in order.

use alloc::vec::Vec;

use super::node::Node;
use super::run::{Content, Entry, Run};

pub fn place_children<'a>(
    children: &[Node<'a>],
    parent: usize,
    runs: &mut Vec<Run<'a>>,
    next: &mut u32,
    cluster_bytes: usize,
) -> Vec<Entry<'a>> {
    let mut entries = Vec::with_capacity(children.len());
    for child in children {
        let idx = runs.len();
        match child {
            Node::Dir { name, children } => {
                let clusters =
                    clusters_for(dir_slots_needed(children.len(), true) * 32, cluster_bytes);
                let first = *next;
                *next += clusters;
                runs.push(Run {
                    first_cluster: first,
                    clusters,
                    content: Content::Dir(Vec::new()),
                    parent: Some(parent),
                });
                let sub = place_children(children, idx, runs, next, cluster_bytes);
                runs[idx].content = Content::Dir(sub);
                entries.push(Entry { name, is_dir: true, run: idx, size: 0 });
            }
            Node::File { name, data } => {
                let clusters = clusters_for(data.len(), cluster_bytes);
                let first = if clusters == 0 { 0 } else { *next };
                *next += clusters;
                runs.push(Run {
                    first_cluster: first,
                    clusters,
                    content: Content::File(data),
                    parent: Some(parent),
                });
                entries.push(Entry { name, is_dir: false, run: idx, size: data.len() as u32 });
            }
        }
    }
    entries
}

/// Slots a directory needs: one per child, `.` and `..` except in the
/// root, and one zero slot so a reader knows where the listing ends.
pub fn dir_slots_needed(children: usize, has_dots: bool) -> usize {
    children + if has_dots { 2 } else { 0 } + 1
}

pub fn clusters_for(bytes: usize, cluster_bytes: usize) -> u32 {
    bytes.div_ceil(cluster_bytes) as u32
}
