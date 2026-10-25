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

//! A private directory for the verifiers' reports and logs, made fresh for
//! one run and removed after it, so no report is written into the tree being
//! judged and no earlier run's report can be read as this one's.

use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Scratch(pub PathBuf);

impl Scratch {
    pub fn new(what: &str) -> Result<Self, String> {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let dir = std::env::temp_dir()
            .join(format!("nonos-harden-{what}-{}-{nanos}", std::process::id()));
        fs::create_dir(&dir).map_err(|e| format!("cannot make {} ({e})", dir.display()))?;
        Ok(Self(dir))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// The last 600 bytes a command printed, on one line.
pub fn tail(log: &Path) -> String {
    let mut buf = Vec::new();
    if let Ok(mut f) = File::open(log) {
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        let _ = f.seek(SeekFrom::Start(len.saturating_sub(600)));
        let _ = f.read_to_end(&mut buf);
    }
    String::from_utf8_lossy(&buf).trim().replace('\n', " | ")
}
