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

//! A file written, read back, sought into and removed through std::fs over
//! the vfs, the way any crate that keeps a cache on disk would.

use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const PATH: &str = "/tmp/std_proof.bin";

pub fn prove() -> Result<String, String> {
    let body: Vec<u8> = (0..4096u32).map(|i| (i * 7 % 256) as u8).collect();
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(PATH)
        .map_err(|e| format!("open {PATH}: {e}"))?;
    f.write_all(&body).map_err(|e| format!("write: {e}"))?;
    f.seek(SeekFrom::Start(1000)).map_err(|e| format!("seek: {e}"))?;
    let mut mid = [0u8; 16];
    f.read_exact(&mut mid).map_err(|e| format!("read after seek: {e}"))?;
    drop(f);
    let whole = fs::read(PATH).map_err(|e| format!("read back: {e}"))?;
    fs::remove_file(PATH).map_err(|e| format!("remove: {e}"))?;
    if whole != body || mid[..] != body[1000..1016] {
        return Err("bytes read back differ from those written".into());
    }
    Ok(format!("{} bytes written, sought to 1000, read back, removed", body.len()))
}
