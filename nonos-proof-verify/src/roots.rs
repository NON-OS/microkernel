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

//! The release roots, the same files the kernel and the loader compile in, so
//! a report can say whether the root it checked against is a release's.

use nonos_attest_path::Kind;

/// A committed release root: what it is called, where it lives in the tree,
/// which kind of slot it holds, and its bytes.
pub struct Release {
    pub name: &'static str,
    pub file: &'static str,
    pub kind: Kind,
    pub root: [u8; 32],
}

pub const RELEASE: [Release; 3] = [
    Release {
        name: "capsule policy root",
        file: "nonos-data/trust/policy/zk_capsule_policy_root.bin",
        kind: Kind::Capsule,
        root: *include_bytes!("../../nonos-data/trust/policy/zk_capsule_policy_root.bin"),
    },
    Release {
        name: "kernel attestation root",
        file: "nonos-data/trust/policy/kernel_attest_root.bin",
        kind: Kind::Kernel,
        root: *include_bytes!("../../nonos-data/trust/policy/kernel_attest_root.bin"),
    },
    Release {
        name: "bootloader attestation root",
        file: "nonos-data/trust/policy/bootloader_attest_root.bin",
        kind: Kind::Bootloader,
        root: *include_bytes!("../../nonos-data/trust/policy/bootloader_attest_root.bin"),
    },
];

/// The release root `root` is, if any.
pub fn release_of(root: &[u8; 32]) -> Option<&'static Release> {
    RELEASE.iter().find(|r| &r.root == root)
}
