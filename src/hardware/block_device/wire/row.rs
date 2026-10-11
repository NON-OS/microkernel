// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The block wire's row of the block layer's table: one disk per endpoint,
//! keyed by the endpoint's index.

use super::super::backend::Backend;
use super::super::disk::Disk;
use super::map::map_wire_error;
use super::{ops, read, table, write};

pub(in super::super) static WIRE: Disk = Disk {
    shown: |k| table::shown(k as u8),
    announced: |k| table::shown(k as u8),
    capacity: |k| ops::capacity(k as u8).map_err(map_wire_error),
    read: |k, lba, out| read::read_blocks(k as u8, lba, out).map_err(map_wire_error),
    write: |k, lba, data| write::write_blocks(k as u8, lba, data).map_err(map_wire_error),
    flush: |k| ops::flush(k as u8).map_err(map_wire_error),
    fits: |k| ops::sectors_fit(k as u8),
    said: |k| table::said(k as u8),
    copy_answers: false,
    report: None,
};

/// Every endpoint's disk, in table order.
pub(in super::super) fn rows() -> impl Iterator<Item = Backend> {
    (0..table::count()).map(|i| Backend::of(&WIRE, i as u32))
}
