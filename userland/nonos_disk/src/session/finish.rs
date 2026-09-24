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

//! After the last job: the partition table, then the flush, then the
//! receipt. The table goes down only once every byte of the volume has,
//! so a disk that lost power mid-install has no table naming a partition
//! whose contents never arrived.

use super::progress::Progress;
use super::step::Session;
use crate::gpt::write_table;
use crate::sink::BlockSink;
use crate::writer::{Receipt, WriteError};

impl<'a> Session<'a> {
    pub(super) fn finish(&mut self, sink: &mut dyn BlockSink) -> Result<Progress<'a>, WriteError> {
        if !self.table_written {
            write_table(sink, &self.plan.layout, self.plan.disk_guid, self.plan.partition_guid)?;
            self.table_written = true;
            return Ok(Progress::TableWritten);
        }
        sink.flush()?;
        Ok(Progress::Done(Receipt {
            disk_guid: self.plan.disk_guid,
            partition_guid: self.plan.partition_guid,
            layout: self.plan.layout,
            geometry: self.plan.geometry,
            bytes_written: self.done,
            files: self.plan.file_runs(),
        }))
    }
}
