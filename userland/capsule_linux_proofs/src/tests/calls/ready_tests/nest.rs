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

//! An epoll may not watch itself through others, or nest too deep.

use crate::linux::file::epoll_rules::{loops, MAX_NESTS};

/// Epolls 10.. as a table: each entry is the descriptors one watches.
fn graph<'a>(edges: &'a [(u64, &'a [u64])]) -> impl Fn(u64) -> Option<Vec<u64>> + 'a {
    move |fd| edges.iter().find(|(e, _)| *e == fd).map(|(_, w)| w.to_vec())
}

#[test]
fn an_epoll_may_not_watch_itself_through_others_or_nest_too_deep() {
    // 11 watches 12, which watches 10: adding 11 to 10 closes a loop.
    let lists = graph(&[(10, &[3]), (11, &[12]), (12, &[10, 4])]);
    assert!(loops(&lists, 10, 11));
    // A diamond is no loop.
    let lists = graph(&[(10, &[]), (11, &[12, 13]), (12, &[14]), (13, &[14]), (14, &[5])]);
    assert!(!loops(&lists, 10, 11));
    // A chain of MAX_NESTS epolls below the target is one too many.
    let deep: Vec<(u64, Vec<u64>)> =
        (0..=MAX_NESTS as u64 + 1).map(|i| (11 + i, vec![12 + i])).collect();
    let table: Vec<(u64, &[u64])> = deep.iter().map(|(e, w)| (*e, w.as_slice())).collect();
    assert!(loops(&graph(&table), 10, 11));
    let table: Vec<(u64, &[u64])> = table.into_iter().take(MAX_NESTS as usize).collect();
    assert!(!loops(&graph(&table), 10, 11));
}
