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

//! Writing the table: MBR, primary header and array, backup array and
//! header. The partition contents are the filesystem's business; this only
//! names where they are.

use super::entry::build_array;
use super::header::{build, Which};
use super::layout::Layout;
use super::mbr::protective;
use crate::crc32::crc32;
use crate::guid::Guid;
use crate::sink::{BlockSink, SinkError};

pub fn write_table(
    sink: &mut dyn BlockSink,
    layout: &Layout,
    disk_guid: Guid,
    partition_guid: Guid,
) -> Result<(), SinkError> {
    let array = build_array(layout, partition_guid);
    let array_crc = crc32(&array);
    let primary = build(layout, disk_guid, array_crc, Which::Primary);
    let backup = build(layout, disk_guid, array_crc, Which::Backup);
    /*
     * Back to front on purpose: the backup pair lands first, the primary
     * header last. A write that dies half way leaves a disk with no valid
     * primary, which firmware reads as "no table" and refuses to boot,
     * rather than a primary that names a partition whose backup and
     * contents never arrived.
     */
    sink.write_at(layout.backup_array_lba, &array)?;
    sink.write_at(layout.backup_header_lba, &backup)?;
    sink.write_at(2, &array)?;
    sink.write_at(0, &protective(layout.total_sectors))?;
    sink.write_at(1, &primary)
}
