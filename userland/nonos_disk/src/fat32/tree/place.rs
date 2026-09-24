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

//! Placement is a depth-first walk that hands out clusters in order: the
//! root takes cluster 2, then each directory or file takes the next run.
//! A pure function of the tree and the cluster size, so the same tree on
//! the same geometry lands on the same sectors every time.

use alloc::vec::Vec;

use super::children::{clusters_for, dir_slots_needed, place_children};
use super::node::Node;
use super::run::{Content, Placed, Run};

pub fn place_with<'a>(tree: &[Node<'a>], cluster_bytes: usize) -> Placed<'a> {
    let mut runs: Vec<Run<'a>> = Vec::new();
    // The root has no `.` and `..`, and its run is reserved before its
    // children so it is cluster 2, where the boot sector says it is.
    let root_clusters = clusters_for(dir_slots_needed(tree.len(), false) * 32, cluster_bytes);
    runs.push(Run {
        first_cluster: 2,
        clusters: root_clusters,
        content: Content::Dir(Vec::new()),
        parent: None,
    });
    let mut next = 2 + root_clusters;
    let entries = place_children(tree, 0, &mut runs, &mut next, cluster_bytes);
    runs[0].content = Content::Dir(entries);
    Placed { runs, data_clusters_needed: (next - 2) as u64 }
}
