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

//! /data goes to the volume before VFS_OP_LOCK, so that spin lock is
//! held over the in-kernel ramfs only, and every hold of it is marked.

use crate::proofs::proofs_8::transport_takeover::text::{between, code, tail};

const VFS: &str = include_str!("../../../../src/fs/vfs/vfs_file.rs");

#[test]
fn g11_data_goes_to_the_volume_before_vfs_op_lock() {
    for (from, to) in [("pub fn write_file(", "pub fn stat("), ("pub fn stat(", "")] {
        let f = if to.is_empty() { tail(VFS, from) } else { between(VFS, from, to) };
        let data = f.find("data_rel(").expect("data");
        assert!(data < f.find("VFS_OP_LOCK").expect("lock"), "{from}: /data after VFS_OP_LOCK");
    }
    let c = code(VFS);
    let marked = c.matches("(VFS_OP_LOCK.lock(),crate::services::lifecycle::quiet())").count();
    assert_eq!((c.matches("VFS_OP_LOCK.lock()").count(), marked), (4, 4));
}
