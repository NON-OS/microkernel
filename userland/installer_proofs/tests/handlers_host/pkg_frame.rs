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

//! Request frames for the pkg handlers, the store paths they install to, and the state each test starts from.

use crate::frame::frame;
use crate::handlers::{pkg_commit::pkg_commit, pkg_query::pkg_query, pkg_remove::pkg_remove};
use crate::protocol::{decode_request, Request, OP_PKG_COMMIT, OP_PKG_QUERY, OP_PKG_REMOVE};
use crate::verify::INSTALL_NAME;
use crate::{clients::vfs, services, syscall, verify};

pub const PATH: &[u8] = b"/downloads/demo_pkg.npkg";
pub const EACCES: i32 = -13;
/// The artifacts pkg_commit installs, in the order it installs them.
pub const STORED: [&[u8]; 4] =
    [b".elf", b".manifest.bin", b".nonos_id_cert.bin", b".zk_trailer.bin"];

/// No files, an empty store, verify answering 0, no service taken.
pub fn fresh() {
    vfs::reset();
    vfs::store_reset();
    verify::reset(0);
    services::reset(&[]);
    syscall::reset(0);
}

pub fn stored(ext: &[u8]) -> Vec<u8> {
    [b"/capsules/", INSTALL_NAME, ext].concat()
}

pub fn digest(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

/// len(2, LE) | bytes: a path, or pkg_remove's name.
fn counted(bytes: &[u8]) -> Vec<u8> {
    [&(bytes.len() as u16).to_le_bytes()[..], bytes].concat()
}

fn handle(op: u16, payload: &[u8], handler: fn(Request<'_>) -> Vec<u8>) -> Vec<u8> {
    let f = frame(op, payload);
    let req = decode_request(&f).expect("frame decodes");
    assert_eq!(req.op, op);
    handler(req)
}

pub fn query(path: &[u8]) -> Vec<u8> {
    handle(OP_PKG_QUERY, &counted(path), pkg_query)
}

pub fn commit(digest: &[u8; 32], path: &[u8]) -> Vec<u8> {
    handle(OP_PKG_COMMIT, &[&digest[..], &counted(path)].concat(), pkg_commit)
}

pub fn remove(name: &[u8]) -> Vec<u8> {
    handle(OP_PKG_REMOVE, &counted(name), pkg_remove)
}
