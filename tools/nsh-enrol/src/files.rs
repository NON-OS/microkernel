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

//! Reading and writing. A file is written whole or not at all, and never
//! over one that exists: the bytes go to a fresh temporary file beside it,
//! which is linked into place only if the name is still free. A secret is
//! created readable by its owner alone.

use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

use crate::Error;

fn io(path: &str, e: std::io::Error) -> Error {
    Error::Io(format!("{path}: {e}"))
}

pub fn read(path: &str) -> Result<Vec<u8>, Error> {
    std::fs::read(path).map_err(|e| io(path, e))
}

pub fn write_new(path: &str, bytes: &[u8], mode: u32) -> Result<(), Error> {
    let part = format!("{path}.part-{}", std::process::id());
    let written = (|| {
        let mut f = OpenOptions::new().write(true).create_new(true).mode(mode).open(&part)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        std::fs::hard_link(&part, path)
    })();
    let _ = std::fs::remove_file(&part);
    written.map_err(|e| io(path, e))
}

/// Exactly `N` bytes, or the file is refused.
pub fn read_exact<const N: usize>(path: &str) -> Result<Box<[u8; N]>, Error> {
    let b = read(path)?;
    let n = b.len();
    b.into_boxed_slice().try_into().map_err(|_| Error::Io(format!("{path}: {n} bytes, not {N}")))
}

/// Milliseconds since the epoch, by this machine's clock.
pub fn now_ms() -> Result<u64, Error> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
    let ms = now.map_err(|_| Error::Value("clock"))?.as_millis();
    u64::try_from(ms).map_err(|_| Error::Value("clock"))
}
