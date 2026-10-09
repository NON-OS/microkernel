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

//! After the last job, which is the primary GPT header: the flush, then the
//! receipt. The receipt keeps every job but the wipe, so the read-back
//! compares every sector that stays written against the bytes that were
//! sent.

use super::progress::Progress;
use super::step::Session;
use crate::sink::BlockSink;
use crate::writer::{Receipt, WriteError};

impl<'a> Session<'a> {
    pub(super) fn finish(&mut self, sink: &mut dyn BlockSink) -> Result<Progress<'a>, WriteError> {
        self.last = Some(super::step::Attempt::Flush);
        sink.flush()?;
        let kept = self.kept_from.min(self.jobs.len());
        Ok(Progress::Done(Receipt {
            disk_guid: self.plan.ids.disk,
            partitions: self.plan.ids.partitions,
            layout: self.plan.layout,
            geometry: self.plan.geometry,
            bytes_written: self.done,
            store_files: self.plan.store.files,
            files: self.plan.file_runs(),
            written: self.jobs.split_off(kept),
        }))
    }
}
