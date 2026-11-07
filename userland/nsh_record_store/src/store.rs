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

//! The directory records are kept in: made owner-only if it is new, and
//! refused if it is not owner-only, since a record holds every command and,
//! by a role's flags, its output.

use crate::files::SessionFiles;
use std::io::{self, Error, ErrorKind};
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};

pub struct Store {
    dir: PathBuf,
}

impl Store {
    pub fn open(dir: impl AsRef<Path>) -> io::Result<Store> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir)?;
        let meta = std::fs::symlink_metadata(&dir)?;
        if !meta.is_dir() || meta.permissions().mode() & 0o077 != 0 {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                "the record directory is not owner-only",
            ));
        }
        Ok(Store { dir })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// The record and checkpoint paths for `session_id`.
    pub fn paths(&self, session_id: &[u8; 16]) -> (PathBuf, PathBuf) {
        let name: String = session_id.iter().map(|b| format!("{b:02x}")).collect();
        (self.dir.join(format!("{name}.record")), self.dir.join(format!("{name}.ckpts")))
    }

    pub(crate) fn create(&self, session_id: &[u8; 16]) -> io::Result<SessionFiles> {
        let (record, ckpts) = self.paths(session_id);
        SessionFiles::create(&record, &ckpts)
    }
}
