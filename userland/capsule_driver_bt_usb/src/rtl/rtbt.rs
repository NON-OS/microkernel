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

//! The second patch file layout. After the signature "RTBTCore" come an
//! eight-octet firmware version and a four-octet section count, then the
//! sections, each an opcode (4 octets), a length (4) and a body. The image
//! is the data of every subsection meant for this ROM, ordered by its
//! priority; nothing is written into it, unlike the first layout.

use alloc::vec::Vec;

use super::refused::Layout;
use super::rtbt_sub::{subsections, Pick};

pub const SIGNATURE: &[u8; 8] = b"RTBTCore";
/// Signature, firmware version and section count.
pub const HEADER: usize = 20;
pub const SNIPPETS: u32 = 1;
pub const DUMMY: u32 = 2;
pub const SECURITY: u32 = 3;

pub(super) fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// The image for a ROM of version `rom` whose security key id is `key`.
pub fn image_for(file: &[u8], rom: u8, key: u8) -> Result<Vec<u8>, Layout> {
    if file.len() < HEADER || &file[..8] != SIGNATURE {
        return Err(Layout::Signature);
    }
    let mut picked: Vec<Pick<'_>> = Vec::new();
    let mut at = HEADER;
    for _ in 0..le32(file, 16) {
        // A section past the end is a damaged file, not the end of the
        // list: the pin already fixed every octet, so none should be cut.
        let head = file.get(at..at + 8).ok_or(Layout::Table)?;
        let (opcode, len) = (le32(head, 0), le32(head, 4) as usize);
        let body = file.get(at + 8..).and_then(|r| r.get(..len)).ok_or(Layout::Bounds)?;
        at += 8 + len;
        let key = match opcode {
            SNIPPETS | DUMMY => None,
            // A chip without a key takes no security section; one with a
            // key takes only the subsections signed for it.
            SECURITY if key != 0 => Some(key),
            _ => continue,
        };
        subsections(body, rom, key, &mut picked)?;
    }
    let image: Vec<u8> = picked.iter().flat_map(|p| p.data.iter().copied()).collect();
    // Nothing chosen, or only empty subsections: there is no patch to send.
    if image.is_empty() {
        return Err(Layout::NoPatch);
    }
    Ok(image)
}
