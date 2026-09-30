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

//! The table as writes, in the order they land. The session queues these
//! last, after every partition's contents and after the store header and
//! the plan, and it wiped the old tables first: a write that stops before
//! here leaves a disk with no table at all.

use alloc::rc::Rc;
use alloc::vec::Vec;

use super::entry::build_array;
use super::header::{build, Which};
use super::mbr::protective;
use crate::crc32::crc32;
use crate::guid::Guid;
use crate::layout::Layout;

pub fn table(layout: &Layout, disk: Guid, unique: &[Guid; 4]) -> Vec<(u64, Rc<Vec<u8>>)> {
    let array = Rc::new(build_array(layout, unique));
    let crc = crc32(&array);
    let backup = Rc::new(build(layout, disk, crc, Which::Backup));
    let primary = Rc::new(build(layout, disk, crc, Which::Primary));
    /*
     * Back to front: the backup pair first, the primary header last. A
     * write that stops between them leaves a valid backup that names only
     * partitions whose contents are already down, which a firmware that
     * falls back to the backup may use.
     */
    alloc::vec![
        (layout.backup_array_lba, array.clone()),
        (layout.backup_header_lba, backup),
        (2, array),
        (0, Rc::new(protective(layout.total_sectors))),
        (1, primary),
    ]
}
