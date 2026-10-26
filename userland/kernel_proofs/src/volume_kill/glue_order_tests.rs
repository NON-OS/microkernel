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

//! The disk order the model reads, pinned at its ends (V1 to V4): the free
//! count moves before an allocation returns, a commit moves the mount
//! before it writes, and nothing outside commit reads the header slot,
//! copies the mount, or writes a tree's node or header from a stream.

use std::path::{Path, PathBuf};

use crate::proofs::proofs_8::transport_takeover::text::{between, code, tail};

const ALLOC: &str = include_str!("../../../../src/fs/blockfs/alloc_block.rs");
const COMMIT: &str = include_str!("../../../../src/fs/blockfs/commit.rs");

/// Every .rs file under src/`dir`, as (path, code alone).
pub(super) fn files(dir: &str) -> Vec<(PathBuf, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src").join(dir);
    let (mut todo, mut out) = (vec![root], Vec::new());
    while let Some(d) = todo.pop() {
        for p in std::fs::read_dir(&d).expect("reads").map(|e| e.expect("entry").path()) {
            if p.is_dir() {
                todo.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let c = code(&std::fs::read_to_string(&p).expect("reads"));
                out.push((p, c));
            }
        }
    }
    assert!(!out.is_empty(), "src/{dir} read");
    out
}

#[test]
fn v1_v2_the_mount_moves_before_the_disk() {
    let alloc = between(ALLOC, "pub fn alloc_block(", "Ok(lba)");
    assert!(alloc.ends_with(&code("mount.superblock.free_lba = lba + 1;")), "{alloc}");
    let commit = concat!(
        "pub fn commit(key: &[u8; 32], mount: &mut BlockFsMount) -> Result<(), BlockFsError> {",
        " mount.superblock.generation += 1; let lba = header_lba(mount.superblock.generation);",
        " mount.header_lba = lba; let payload = serialize(&mount.superblock);",
        " crate::fs::cryptoblock::write(key, lba, &payload).map_err(BlockFsError::CryptoBlock) }",
    );
    assert_eq!(tail(COMMIT, "pub fn commit("), code(commit));
}

#[test]
fn v3_v4_no_slot_read_no_mount_copy_no_stream_commit() {
    for (p, c) in files("fs") {
        let name = p.display().to_string();
        if !name.ends_with("blockfs/commit.rs") {
            assert!(!c.contains(".header_lba"), "{name} reads the header slot");
        }
        for copy in ["=s.mount", "=state.mount", "=v.mount", "BlockFsMount=*"] {
            assert!(!c.contains(copy), "{name} copies the mount");
        }
        let stream = name.contains("blockfs/file_stream") || name.contains("blockfs/tree_writer");
        for call in ["cryptoblock::write(", "write_node(", "commit("] {
            assert!(!(stream && c.contains(call)), "{name} calls {call}");
        }
    }
}
