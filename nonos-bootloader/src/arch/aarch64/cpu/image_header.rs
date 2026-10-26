// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The arm64 Image header at the kernel's first bytes (start.S writes it):
//! 64 bytes, image_size the little-endian word at 16 and the magic
//! "ARM\x64" at 56. image_size counts BSS and the boot stack, which no ELF
//! segment covers.

const MAGIC: [u8; 4] = *b"ARM\x64";

/// The image_size the header claims; None when `head` is short, has no
/// magic, or claims nothing.
pub fn image_size(head: &[u8]) -> Option<u64> {
    if head.get(56..60)? != MAGIC {
        return None;
    }
    let size = u64::from_le_bytes(head.get(16..24)?.try_into().ok()?);
    (size != 0).then_some(size)
}
