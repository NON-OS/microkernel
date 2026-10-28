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

//! Bytes placed by install, and the kernel's own staged files, against the
//! clients' budget (`capsule_vfs/src/store/fdtable/budget.rs`).

use super::{file, fill, A, KERNEL, MIB};
use crate::vfs_store::{Store, StoreError, DATA_BYTES_MAX};

#[test]
fn installing_bytes_is_held_to_the_budget_too() {
    let mut s = Store::new();
    for i in 0..2 {
        assert_eq!(put(&mut s, &format!("/tmp/i{i}"), 64 * MIB), 64 * MIB);
    }
    let big = vec![7u8; 40 * MIB];
    assert_eq!(s.install_bytes("/pkg/x", 0, &big, A), Err(StoreError::Full));
    assert!(s.install_bytes("/pkg/y", 0, b"small", A).is_ok());
}

#[test]
fn the_kernels_staged_files_leave_clients_the_whole_budget() {
    let mut s = Store::new();
    let staged = vec![1u8; 60 * MIB];
    assert!(s.install_bytes("/capsules/staged", 0, &staged, KERNEL).is_ok());
    let got: usize = (0..3).map(|i| fill(&mut s, &format!("/tmp/c{i}"), A, 8 * MIB)).sum();
    assert!(got >= 2 * 64 * MIB, "clients got {got} bytes beside the staged files");
    assert!(s.held_except(None) <= DATA_BYTES_MAX);
}

/// Write `bytes` to a new file of A's in 8 MiB pieces, stopping at a refusal.
pub(super) fn put(s: &mut Store, path: &str, bytes: usize) -> usize {
    let fd = file(s, path, A);
    let piece = vec![0x5Au8; 8 * MIB];
    let mut written = 0;
    while written < bytes {
        let Ok(n) = s.write(fd, A, &piece[..(bytes - written).min(8 * MIB)]) else { break };
        written += n as usize;
    }
    written
}
