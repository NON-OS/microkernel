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

//! Linux's own answer: the same call on a real directory of the host.

use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::unix::fs::{FileExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use nonos_user_files::wire::flags::{APPEND, CREATE, DIRECTORY, EXCL, READ, TRUNC, WRITE};

pub type Got<T> = Result<T, i32>;

fn errno<T>(r: io::Result<T>) -> Got<T> {
    r.map_err(|e| e.raw_os_error().unwrap_or(-1))
}

/// A host directory standing in for Documents.
pub struct Reference {
    pub root: PathBuf,
}

impl Reference {
    pub fn new(tag: u64) -> Reference {
        let root =
            std::env::temp_dir().join(format!("user_files_ref_{}_{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("a temporary directory");
        Reference { root }
    }

    pub fn at(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    pub fn open(&self, rel: &str, flags: u32) -> Got<File> {
        let bits = [
            (CREATE, 0o100),
            (EXCL, 0o200),
            (TRUNC, 0o1000),
            (APPEND, 0o2000),
            (DIRECTORY, 0o200000),
        ];
        let custom = bits.iter().filter(|(f, _)| flags & f != 0).fold(0, |a, (_, b)| a | b);
        let mut o = OpenOptions::new();
        o.read(flags & READ != 0).write(flags & WRITE != 0).custom_flags(custom).mode(0o644);
        errno(o.open(self.at(rel)))
    }

    /// `open` with Linux's own flags, as a guest passes them.
    pub fn open_linux(&self, rel: &str, flags: u64) -> Got<File> {
        let access = flags & 3;
        let mut o = OpenOptions::new();
        o.read(access != 1).write(access != 0).custom_flags((flags & !3) as i32).mode(0o644);
        errno(o.open(self.at(rel)))
    }

    pub fn read(f: &File, offset: u64, len: usize) -> Got<Vec<u8>> {
        let mut out = vec![0u8; len];
        if len == 0 {
            // A read of nothing still asks, and is refused as any read is.
            errno(f.read_at(&mut out, offset))?;
        }
        let mut at = 0;
        while at < len {
            match errno(f.read_at(&mut out[at..], offset + at as u64))? {
                0 => break,
                n => at += n,
            }
        }
        out.truncate(at);
        Ok(out)
    }

    pub fn write(f: &File, offset: u64, data: &[u8]) -> Got<usize> {
        errno(f.write_at(data, offset))
    }

    pub fn ftruncate(f: &File, size: u64) -> Got<()> {
        errno(f.set_len(size))
    }

    pub fn fsync(f: &File) -> Got<()> {
        errno(f.sync_all())
    }

    /// Whether a directory, and the size.
    pub fn stat(&self, rel: &str) -> Got<(bool, u64)> {
        let m = errno(fs::metadata(self.at(rel)))?;
        Ok((m.is_dir(), if m.is_dir() { 0 } else { m.len() }))
    }

    pub fn mkdir(&self, rel: &str) -> Got<()> {
        errno(fs::create_dir(self.at(rel)))
    }

    pub fn rmdir(&self, rel: &str) -> Got<()> {
        errno(fs::remove_dir(self.at(rel)))
    }

    pub fn unlink(&self, rel: &str) -> Got<()> {
        errno(fs::remove_file(self.at(rel)))
    }

    pub fn rename(&self, from: &str, to: &str) -> Got<()> {
        errno(fs::rename(self.at(from), self.at(to)))
    }

    pub fn truncate(&self, rel: &str, size: u64) -> Got<()> {
        errno(OpenOptions::new().write(true).open(self.at(rel)).and_then(|f| f.set_len(size)))
    }

    pub fn list(&self, rel: &str) -> Got<Vec<Vec<u8>>> {
        let mut names: Vec<Vec<u8>> = errno(fs::read_dir(self.at(rel)))?
            .map(|e| e.map(|e| e.file_name().into_encoded_bytes()))
            .collect::<io::Result<_>>()
            .map_err(|e| e.raw_os_error().unwrap_or(-1))?;
        names.sort();
        Ok(names)
    }
}

impl Drop for Reference {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(Path::new(&self.root));
    }
}
