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

//! ramfs requests as the kernel's client frames them
//! (`src/fs/ramfs_capsule/protocol/encode.rs`): a sequence number, the op,
//! a zero word, then the op's fields; and its replies, a sequence number and
//! a status.

use ramfs_host::Ramfs;

pub const OPEN: u16 = 1;
pub const CLOSE: u16 = 2;
pub const READ: u16 = 3;
pub const WRITE: u16 = 4;
pub const TRUNCATE: u16 = 5;
pub const CREATE: u32 = 1;
/// A user process, as the kernel stamps one on a request it forwards.
pub const PID: u32 = 7;

pub fn frame(op: u16, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len());
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(payload);
    out
}

/// The reply's status and payload.
pub fn call(fs: &mut Ramfs, op: u16, payload: &[u8]) -> (i32, Vec<u8>) {
    let reply = fs.serve(&frame(op, payload), PID).expect("answered");
    let status = i32::from_le_bytes([reply[4], reply[5], reply[6], reply[7]]);
    (status, reply[8..].to_vec())
}

pub fn open(fs: &mut Ramfs, path: &str) -> u64 {
    let mut p = CREATE.to_le_bytes().to_vec();
    p.extend_from_slice(&(path.len() as u16).to_le_bytes());
    p.extend_from_slice(path.as_bytes());
    let (status, body) = call(fs, OPEN, &p);
    assert_eq!(status, 0, "open {path}");
    u64::from_le_bytes(body[..8].try_into().unwrap())
}

pub fn open_status(fs: &mut Ramfs, path: &str) -> i32 {
    let mut p = CREATE.to_le_bytes().to_vec();
    p.extend_from_slice(&(path.len() as u16).to_le_bytes());
    p.extend_from_slice(path.as_bytes());
    call(fs, OPEN, &p).0
}

pub fn write(fs: &mut Ramfs, h: u64, offset: u64, data: &[u8]) -> i32 {
    let mut p = h.to_le_bytes().to_vec();
    p.extend_from_slice(&offset.to_le_bytes());
    p.extend_from_slice(data);
    call(fs, WRITE, &p).0
}

pub fn truncate(fs: &mut Ramfs, h: u64, len: u64) -> i32 {
    let mut p = h.to_le_bytes().to_vec();
    p.extend_from_slice(&len.to_le_bytes());
    call(fs, TRUNCATE, &p).0
}

/// The whole file, read in one request.
pub fn read_all(fs: &mut Ramfs, h: u64) -> Vec<u8> {
    let mut p = h.to_le_bytes().to_vec();
    p.extend_from_slice(&0u64.to_le_bytes());
    p.extend_from_slice(&u32::MAX.to_le_bytes());
    let (status, body) = call(fs, READ, &p);
    assert!(status >= 0, "read {status}");
    body
}
