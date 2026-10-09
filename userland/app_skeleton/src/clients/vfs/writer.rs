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


//! A file written as its bytes arrive, for a caller that never holds the
//! whole of it: a download, written a piece at a time on one open fd whose
//! position the server advances, and closed when the writer is dropped.

use alloc::{vec, vec::Vec};

use crate::wire::{read_u32, HDR_LEN};

/// Bytes per write request, under the server's payload cap (write_file.rs).
const CHUNK: usize = 60 * 1024;

pub struct VfsWriter {
    port: u32,
    owner_pid: u32,
    fd: u32,
    path: Vec<u8>,
    rx: Vec<u8>,
    written: u64,
}

impl VfsWriter {
    /// Create `path`, or empty it if it is there, and open it for writing.
    pub fn create(owner_pid: u32, path: &[u8]) -> Result<VfsWriter, &'static str> {
        Self::open(owner_pid, path, super::types::O_CREATE | super::types::O_TRUNC)
    }

    /// Open `path` to go on writing at its end, creating it if it is not
    /// there: a download resumed from the part that came before.
    pub fn reopen(owner_pid: u32, path: &[u8]) -> Result<VfsWriter, &'static str> {
        let mut w = Self::open(owner_pid, path, super::types::O_CREATE)?;
        let (size, _) = super::stat::stat(owner_pid, path)?;
        w.seek(size)?;
        w.written = size;
        Ok(w)
    }

    fn open(owner_pid: u32, path: &[u8], flags: u32) -> Result<VfsWriter, &'static str> {
        if path.is_empty() || path.len() > 255 {
            return Err("vfs path invalid");
        }
        let port = super::resolve::vfs_port();
        let mut open = Vec::with_capacity(9 + path.len());
        open.extend_from_slice(&owner_pid.to_le_bytes());
        open.push(path.len() as u8);
        open.extend_from_slice(path);
        open.extend_from_slice(&flags.to_le_bytes());
        let mut rx = vec![0u8; HDR_LEN + 16];
        let (status, total) = super::call::call(port, super::types::OP_OPEN, 5, &open, &mut rx)?;
        if status != 0 || total < HDR_LEN + 8 {
            return Err(super::errmsg::errmsg(status));
        }
        let fd = read_u32(&rx, HDR_LEN + 4)?;
        Ok(VfsWriter { port, owner_pid, fd, path: Vec::from(path), rx, written: 0 })
    }

    /// Bytes written so far.
    pub fn written(&self) -> u64 {
        self.written
    }

    /// Append `data` at the end of what is written.
    pub fn append(&mut self, data: &[u8]) -> Result<(), &'static str> {
        for chunk in data.chunks(CHUNK) {
            let mut write = Vec::with_capacity(8 + chunk.len());
            write.extend_from_slice(&self.owner_pid.to_le_bytes());
            write.extend_from_slice(&self.fd.to_le_bytes());
            write.extend_from_slice(chunk);
            let (status, total) =
                super::call::call(self.port, super::types::OP_WRITE, 6, &write, &mut self.rx)?;
            if status != 0 || total < HDR_LEN + 8 {
                return Err(super::errmsg::errmsg(status));
            }
            if read_u32(&self.rx, HDR_LEN + 4)? as usize != chunk.len() {
                return Err("vfs short write");
            }
            self.written += chunk.len() as u64;
        }
        Ok(())
    }

    /// Throw away what is written and start the file again from its first
    /// byte: a server that answered a resume with the whole file.
    pub fn restart(&mut self) -> Result<(), &'static str> {
        super::truncate::truncate(self.owner_pid, &self.path, 0)?;
        self.seek(0)?;
        self.written = 0;
        Ok(())
    }

    fn seek(&mut self, to: u64) -> Result<(), &'static str> {
        let mut body = [0u8; 17];
        body[..4].copy_from_slice(&self.owner_pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        body[8] = super::types::SEEK_SET;
        body[9..17].copy_from_slice(&to.to_le_bytes());
        let (status, _) =
            super::call::call(self.port, super::types::OP_SEEK, 5, &body, &mut self.rx)?;
        if status != 0 {
            return Err("vfs seek failed");
        }
        Ok(())
    }
}

impl Drop for VfsWriter {
    fn drop(&mut self) {
        let mut close = [0u8; 8];
        close[..4].copy_from_slice(&self.owner_pid.to_le_bytes());
        close[4..8].copy_from_slice(&self.fd.to_le_bytes());
        let _ = super::call::call(self.port, super::types::OP_CLOSE, 7, &close, &mut self.rx);
    }
}
