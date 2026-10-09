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

//! The keys Alpine signs x86_64 indexes with, byte for byte as the
//! distribution publishes them in its alpine-keys package.

use alloc::vec::Vec;

use super::base64::decode;

const KEYS: [(&[u8], &[u8]); 3] = [
    (
        b"alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub",
        include_bytes!(
            "../../../../keys/alpine/alpine-devel@lists.alpinelinux.org-4a6a0840.rsa.pub"
        ),
    ),
    (
        b"alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub",
        include_bytes!(
            "../../../../keys/alpine/alpine-devel@lists.alpinelinux.org-5261cecb.rsa.pub"
        ),
    ),
    (
        b"alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub",
        include_bytes!(
            "../../../../keys/alpine/alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub"
        ),
    ),
];

/// The DER public key a signature entry names, if it is one trusted here.
pub fn spki(name: &[u8]) -> Option<Vec<u8>> {
    let (_, pem) = KEYS.iter().find(|(n, _)| *n == name)?;
    let body: Vec<u8> = pem
        .split(|&c| c == b'\n')
        .filter(|line| !line.starts_with(b"-----"))
        .flat_map(|line| line.iter().copied().filter(|c| !c.is_ascii_whitespace()))
        .collect();
    decode(&body)
}
