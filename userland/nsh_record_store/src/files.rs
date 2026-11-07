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

//! One session's two files, created together and appended to in order.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub(crate) struct SessionFiles {
    record: File,
    ckpts: File,
}

impl SessionFiles {
    /// Both files, new and owner-only. If the checkpoint file cannot be
    /// made, the record file just made is removed again.
    pub(crate) fn create(record: &Path, ckpts: &Path) -> io::Result<SessionFiles> {
        let rec = create_new(record)?;
        match create_new(ckpts) {
            Ok(ck) => Ok(SessionFiles { record: rec, ckpts: ck }),
            Err(e) => {
                let _ = std::fs::remove_file(record);
                Err(e)
            }
        }
    }

    pub(crate) fn append_record(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.record.write_all(bytes)
    }

    /// The record bytes so far reach the disk first, then the checkpoint.
    pub(crate) fn append_ckpt(&mut self, ckpt: &[u8]) -> io::Result<()> {
        self.record.sync_data()?;
        self.ckpts.write_all(ckpt)?;
        self.ckpts.sync_data()
    }

    pub(crate) fn sync(&mut self) -> io::Result<()> {
        self.record.sync_all()?;
        self.ckpts.sync_all()
    }
}

fn create_new(path: &Path) -> io::Result<File> {
    OpenOptions::new().append(true).create_new(true).mode(0o600).open(path)
}
