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

//! The block wire's row of the block layer's table: one disk per unit of
//! each endpoint, keyed endpoint << 16 | unit.

use super::super::backend::Backend;
use super::super::disk::Disk;
use super::map::map_wire_error;
use super::target::units;
use super::{ops, read, table, write};

const fn key(index: u8, unit: u16) -> u32 {
    (index as u32) << 16 | unit as u32
}

const fn index(k: u32) -> u8 {
    (k >> 16) as u8
}

const fn unit(k: u32) -> u16 {
    k as u16
}

pub(in super::super) static WIRE: Disk = Disk {
    shown: |k| table::shown(index(k)),
    announced: |k| table::shown(index(k)),
    capacity: |k| ops::capacity(index(k), unit(k)).map_err(map_wire_error),
    read: |k, lba, out| read::read_blocks(index(k), unit(k), lba, out).map_err(map_wire_error),
    write: |k, lba, data| write::write_blocks(index(k), unit(k), lba, data).map_err(map_wire_error),
    flush: |k| ops::flush(index(k), unit(k)).map_err(map_wire_error),
    fits: |k| ops::sectors_fit(index(k), unit(k)),
    said: |k| table::said(index(k), unit(k)),
    copy_answers: false,
    report: None,
};

/// The endpoint a row of the block wire asks; `None` for any other row.
pub(in super::super) fn endpoint_of(b: &Backend) -> Option<u8> {
    core::ptr::eq(b.disk, &WIRE).then_some(index(b.key))
}

/// Every unit of every endpoint, in table order.
pub(in super::super) fn rows() -> impl Iterator<Item = Backend> {
    (0..table::count()).flat_map(|i| {
        let n = table::endpoint(i).map_or(0, units);
        (0..n).map(move |u| Backend::of(&WIRE, key(i, u)))
    })
}
