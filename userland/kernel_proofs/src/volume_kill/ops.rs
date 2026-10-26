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

//! The operations the model runs, each read from its kernel function:
//! create (and the link inside it), ensure_record and a record chain's
//! append; unlink and close are in ops_more.rs.

use super::order::{order, Step::*, Step};

const CREATE: &str = include_str!("../../../../src/fs/blockfs/create.rs");
const LINK: &str = include_str!("../../../../src/fs/blockfs/dir_link.rs");
const ENSURE: &str = include_str!("../../../../src/fs/blockfs/dir_record_alloc.rs");
const ROOM: &str = include_str!("../../../../src/fs/blockfs/dir_room.rs");

/// The slots an operation allocates into: its child, a new record, an
/// index and the tree's data block.
pub(super) const CHILD: u8 = 0;
pub(super) const REC: u8 = 1;
pub(super) const INDEX: u8 = 2;
pub(super) const DATA: u8 = 3;

pub(super) fn create() -> Vec<Step> {
    let n = [
        ("alloc_block(mount)", Alloc(CHILD)),
        ("commit(key, mount)", Commit),
        ("write_node(key, child_lba", Init(CHILD)),
        ("link(key, mount, parent_lba", Link(CHILD)),
    ];
    order(CREATE, "pub fn create(", None, &n)
}

pub(super) fn link(child: u8) -> Vec<Step> {
    let n = [
        ("record_with_room(", Room),
        ("cryptoblock::write(key, rec_lba", PointRec(child)),
        ("write_node(key, dir_lba", WriteDir),
    ];
    order(LINK, "pub fn link(", None, &n)
}

/// ensure_record when the directory has no record yet.
pub(super) fn ensure() -> Vec<Step> {
    let n = [
        ("alloc_block(mount)", Alloc(REC)),
        ("commit(key, mount)", Commit),
        ("cryptoblock::write(key, lba, &empty)", Init(REC)),
        ("dir.first_record_lba = lba", FirstRec(REC)),
    ];
    order(ENSURE, "pub(super) fn ensure_record(", None, &n)
}

/// append when every record is full.
pub(super) fn append() -> Vec<Step> {
    let n = [
        ("alloc_block(mount)", Alloc(REC)),
        ("commit(key, mount)", Commit),
        ("cryptoblock::write(key, lba, &empty)", Init(REC)),
        ("cryptoblock::write(key, after, &prev)", Chain(REC)),
    ];
    order(ROOM, "fn append(", None, &n)
}
