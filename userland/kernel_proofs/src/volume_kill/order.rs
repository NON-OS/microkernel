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

//! Each operation's steps, in the order the kernel source takes them: every
//! needle must occur exactly once in the function's code, and the steps are
//! sorted by where it does. A change of order in the source is a change of
//! order in the model, so the model checks the code as it is.

use crate::proofs::proofs_8::transport_takeover::text::{between, code, tail};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Step {
    /// alloc_block: the free count moves past a new block, held in `slot`.
    Alloc(u8),
    /// commit: the generation moves, then the header goes to the disk.
    Commit,
    /// A new block written whole, before anything points at it.
    Init(u8),
    /// The record with room found, or made (`Room` in link).
    Room,
    /// The record block written with the new child in it.
    PointRec(u8),
    /// The directory node written, naming its first record.
    WriteDir,
    /// ensure_record: the directory's first record set, in memory.
    FirstRec(u8),
    /// append: the previous record written, linking `slot` after it.
    Chain(u8),
    /// The record block written with a child taken out.
    UnpointRec,
    /// A file's node written, naming its index in `slot`.
    PointNode(u8),
    /// The tree's own blocks: data allocated and written, no commit.
    Tree,
    /// create's link: Room, PointRec and WriteDir, in link's order.
    Link(u8),
}

/// The function at `from`, up to `to` or the file's end.
fn body(src: &str, from: &str, to: Option<&str>) -> String {
    to.map_or_else(|| tail(src, from), |to| between(src, from, to))
}

/// `needles` in the order `src`'s function takes them.
pub(super) fn order(
    src: &str,
    from: &str,
    to: Option<&str>,
    needles: &[(&str, Step)],
) -> Vec<Step> {
    let b = body(src, from, to);
    let mut at: Vec<(usize, Step)> = needles
        .iter()
        .map(|&(n, s)| {
            let n = code(n);
            assert_eq!(b.matches(&n).count(), 1, "`{n}` once in `{from}`");
            (b.find(&n).expect("found"), s)
        })
        .collect();
    at.sort_by_key(|&(i, _)| i);
    at.into_iter().map(|(_, s)| s).collect()
}
