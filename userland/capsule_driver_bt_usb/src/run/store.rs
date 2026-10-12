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

//! The vendor's files, read from the file store (vfs_pool) under
//! /firmware/rtl_bt/: opened, read in pieces, closed. Nothing is written,
//! and the files are checked against their pins by the caller.

use alloc::vec::Vec;

use super::store_call::{call, lookup, Call};
use crate::rtl::Blobs;

pub const DIR: &[u8] = b"/firmware/rtl_bt/";
const OP_OPEN: u16 = 1;
const OP_CLOSE: u16 = 2;
const OP_READ: u16 = 3;
/// The store's largest read (its MAX_DATA_BYTES).
const PIECE: usize = 65_536;

pub struct Store;

impl Blobs for Store {
    fn read(&mut self, file: &str, max: usize) -> Option<Vec<u8>> {
        // The store keeps the owner's pid each request names only when the
        // kernel stamped the same; ours is read from our own name, which
        // needs no CoreExec as getpid would (as the keyring's is).
        let (_, me) = lookup(b"driver.bt_usb0")?;
        let mut c = Call::new(lookup(b"vfs_pool")?.0, PIECE);
        let path = [DIR, file.as_bytes()].concat();
        let mut open = me.to_le_bytes().to_vec();
        open.push(u8::try_from(path.len()).ok()?);
        open.extend_from_slice(&path);
        open.extend_from_slice(&0u32.to_le_bytes());
        let fd = call(&mut c, OP_OPEN, &open)?;
        let fd = u32::from_le_bytes(fd.get(..4)?.try_into().ok()?);
        let bytes = pieces(&mut c, me, fd, max);
        let _ = call(&mut c, OP_CLOSE, &[me.to_le_bytes(), fd.to_le_bytes()].concat());
        bytes
    }
}

/// Up to `max` octets, a piece at a time, until a short piece ends the
/// file.
fn pieces(c: &mut Call, me: u32, fd: u32, max: usize) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    while out.len() < max {
        let want = (max - out.len()).min(PIECE);
        let body = [me, fd, want as u32].map(u32::to_le_bytes).concat();
        let got = call(c, OP_READ, &body)?;
        out.extend_from_slice(got);
        if got.len() < want {
            break;
        }
    }
    Some(out)
}
