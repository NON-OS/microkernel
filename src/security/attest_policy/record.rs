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

//! The record's layout, apart from where its values come from, so a host test
//! and a capsule parser read the one definition.
//!
//! ```text
//! 0       version
//! 1       flags: KERNEL_CHECKED, CAPSULE_PATH_ROOT
//! 2       kernel tree depth
//! 3       capsule tree depth
//! 4..8    zero
//! 8..16   boot epoch, u64 little-endian
//! 16..24  policy epoch, u64 little-endian
//! 24..56  kernel root
//! 56..88  capsule root
//! ```
//!
//! A tree that is absent is all zero with its flag clear, never a stand-in
//! root: a verifier must be able to tell "not checked" from a value.

pub const RECORD_VERSION: u8 = 1;
pub const RECORD_LEN: usize = 88;

/// The boot chain's path gate ran and passed against `kernel root`.
pub const KERNEL_CHECKED: u8 = 1 << 0;
/// The spawn gate folds paths against `capsule root`.
pub const CAPSULE_PATH_ROOT: u8 = 1 << 1;

/// One policy tree: its root, the epoch its contexts carry, and its depth.
#[derive(Clone, Copy)]
pub struct Tree {
    pub root: [u8; 32],
    pub epoch: u64,
    pub depth: u8,
}

/// A tree with a zero root was never enrolled and is reported as absent.
fn present(t: Option<&Tree>) -> Option<&Tree> {
    t.filter(|t| t.root != [0u8; 32])
}

pub fn encode(kernel: Option<&Tree>, capsule: Option<&Tree>) -> [u8; RECORD_LEN] {
    let mut r = [0u8; RECORD_LEN];
    r[0] = RECORD_VERSION;
    if let Some(k) = present(kernel) {
        r[1] |= KERNEL_CHECKED;
        r[2] = k.depth;
        r[8..16].copy_from_slice(&k.epoch.to_le_bytes());
        r[24..56].copy_from_slice(&k.root);
    }
    if let Some(c) = present(capsule) {
        r[1] |= CAPSULE_PATH_ROOT;
        r[3] = c.depth;
        r[16..24].copy_from_slice(&c.epoch.to_le_bytes());
        r[56..88].copy_from_slice(&c.root);
    }
    r
}
