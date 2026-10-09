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

//! The `MkAttestPolicy` record, read. The kernel writes it in
//! `src/security/attest_policy/record.rs`; the layout is documented there and a
//! host test encodes with that file and parses with this one.

pub const ATTEST_POLICY_LEN: usize = 88;
const VERSION: u8 = 1;
const KERNEL_CHECKED: u8 = 1 << 0;
const CAPSULE_PATH_ROOT: u8 = 1 << 1;

/// One policy tree as the kernel reports it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PolicyTree {
    pub root: [u8; 32],
    pub epoch: u64,
    pub depth: u8,
}

/// The kernel tree the boot chain checked, and the capsule tree the spawn gate
/// checks. `None` means that gate did not check against a path tree on this
/// boot, which is not the same as any root.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AttestPolicy {
    pub kernel: Option<PolicyTree>,
    pub capsule: Option<PolicyTree>,
}

fn u64_at(r: &[u8], at: usize) -> u64 {
    let mut w = [0u8; 8];
    w.copy_from_slice(&r[at..at + 8]);
    u64::from_le_bytes(w)
}

fn root_at(r: &[u8], at: usize) -> [u8; 32] {
    let mut out = [0u8; 32];
    out.copy_from_slice(&r[at..at + 32]);
    out
}

/*
 * Strict: a version this reader does not know, an unknown flag, a nonzero
 * reserved byte, or a field set under a clear flag is refused, so a record from
 * a newer kernel is never half-read as an older one.
 */
pub fn parse_attest_policy(r: &[u8]) -> Option<AttestPolicy> {
    if r.len() != ATTEST_POLICY_LEN || r[0] != VERSION {
        return None;
    }
    let flags = r[1];
    if flags & !(KERNEL_CHECKED | CAPSULE_PATH_ROOT) != 0 || r[4..8] != [0u8; 4] {
        return None;
    }
    let tree = |on: bool, depth: u8, epoch_off: usize, root_off: usize| {
        let t = PolicyTree { root: root_at(r, root_off), epoch: u64_at(r, epoch_off), depth };
        match on {
            true if t.root != [0u8; 32] => Some(Some(t)),
            false if t == PolicyTree { root: [0u8; 32], epoch: 0, depth: 0 } => Some(None),
            _ => None,
        }
    };
    Some(AttestPolicy {
        kernel: tree(flags & KERNEL_CHECKED != 0, r[2], 8, 24)?,
        capsule: tree(flags & CAPSULE_PATH_ROOT != 0, r[3], 16, 56)?,
    })
}
