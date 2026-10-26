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

//! unlink and a file's close, read from their kernel functions.

use super::ops::INDEX;
use super::order::{order, Step::*, Step};

const UNLINK: &str = include_str!("../../../../src/fs/blockfs/dir_unlink.rs");
const CLOSE: &str = include_str!("../../../../src/fs/blockfs/file_close.rs");

pub(super) fn unlink() -> Vec<Step> {
    let n = [("cryptoblock::write(key, rec", UnpointRec), ("write_node(key, dir_lba", WriteDir)];
    order(UNLINK, "pub(crate) fn unlink(", None, &n)
}

pub(super) fn close() -> Vec<Step> {
    let n = [
        ("tree.finish(", Tree),
        ("store.alloc()", Alloc(INDEX)),
        ("cryptoblock::write(key, index_lba", Init(INDEX)),
        ("commit(key, mount)", Commit),
        ("write_node(key, node_lba", PointNode(INDEX)),
    ];
    order(CLOSE, "pub(super) fn close_file(", None, &n)
}
