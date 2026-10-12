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

//! A blob this capsule may send to a controller, named by its file in
//! linux-firmware and pinned by length and SHA-256 (FIPS 180-4). A file of
//! any other length or digest is refused before one octet of it reaches
//! the controller. The blobs are built into the capsule under its seal,
//! and checked against their pins when it is built and again before each
//! download (Decision 19).

use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq, Eq)]
pub struct Pin {
    /// The file's name in linux-firmware's `rtl_bt/`, and in the tree's
    /// `nonos-bootloader/firmware/rtl_bt/`.
    pub file: &'static str,
    pub len: usize,
    pub sha256: [u8; 32],
}

impl Pin {
    /// Whether `bytes` are exactly the pinned file.
    pub fn holds(&self, bytes: &[u8]) -> bool {
        bytes.len() == self.len && Sha256::digest(bytes).as_slice() == self.sha256
    }
}

/// A digest written as 64 hex digits, read when the capsule is built, so a
/// pin mistyped fails the build rather than refusing the right file.
pub const fn hex32(s: &str) -> [u8; 32] {
    let b = s.as_bytes();
    assert!(b.len() == 64, "a SHA-256 is 64 hex digits");
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        out[i] = nibble(b[2 * i]) << 4 | nibble(b[2 * i + 1]);
        i += 1;
    }
    out
}

const fn nibble(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        _ => panic!("a pin is written in lower-case hex"),
    }
}
