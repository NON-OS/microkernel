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
//! Usage of the crypto and attestation utilities the terminal's caps allow.

pub(crate) const CRYPTO: &[(&[u8], &[u8], &[u8])] = &[
    (b"hex", b"hex [-d] <text...>", b"the bytes as hex, or -d to read hex back to bytes"),
    (b"b64", b"b64 [-d] <text...>", b"base64 of the text, or -d to decode base64"),
    (
        b"hash",
        b"hash [-a sha256|sha512] <text...>",
        b"the digest of the arguments, sha256 by default",
    ),
    (b"rand", b"rand [n]", b"n random bytes from the kernel, as hex; sixteen by default"),
    (
        b"attest",
        b"attest",
        b"the measured-boot state: kernel and program hashes and what verified",
    ),
    (b"pcr", b"pcr", b"the measured-boot hashes alone, one per line"),
];
