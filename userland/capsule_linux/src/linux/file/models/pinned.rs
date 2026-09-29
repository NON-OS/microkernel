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

/*
 * The models this personality vouches for: a name and the SHA-256 the
 * file must have. The table is part of the signed personality, so its
 * measurement covers every digest here; the disk only says where the bytes
 * wait.
 */

pub struct Pinned {
    pub name: &'static [u8],
    pub sha256: [u8; 32],
}

/*
 * Qwen2.5-0.5B-Instruct, Q4_K_M, from Qwen/Qwen2.5-0.5B-Instruct-GGUF on
 * Hugging Face: 491,400,032 bytes, the digest its LFS pointer names.
 */
pub const PINNED: &[Pinned] = &[Pinned {
    name: b"/qwen2.5-0.5b-instruct-q4_k_m.gguf",
    sha256: [
        0x74, 0xa4, 0xda, 0x8c, 0x9f, 0xdb, 0xcd, 0x15, 0xbd, 0x1f, 0x6d, 0x01, 0xd6, 0x21, 0x41,
        0x0d, 0x31, 0xc6, 0xfc, 0x00, 0x98, 0x6f, 0x5e, 0xb6, 0x87, 0x82, 0x4e, 0x7b, 0x93, 0xd7,
        0xa9, 0xdb,
    ],
}];

pub fn pin_of(name: &[u8]) -> Option<&'static [u8; 32]> {
    PINNED.iter().find(|p| p.name == name).map(|p| &p.sha256)
}
