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

//! The load_store payload's head of caps and five lengths, and the offsets they give.

pub(super) const HEAD_LEN: usize = 28;

pub(super) struct Layout {
    pub(super) elf_len: usize,
    pub(super) cert_len: usize,
    pub(super) manifest_len: usize,
    pub(super) trailer_len: usize,
    pub(super) args_len: usize,
    pub(super) after_elf: usize,
    pub(super) after_cert: usize,
    pub(super) after_manifest: usize,
    pub(super) after_trailer: usize,
    pub(super) total: usize,
}

impl Layout {
    /// None when the head is short, an offset overflows, or the payload is
    /// not exactly the head plus the five blobs it names. On a 64-bit usize the
    /// chain cannot overflow, as five u32 lengths plus HEAD_LEN sum far below
    /// usize::MAX, so oversized lengths are refused by the total check instead.
    pub(super) fn parse(p: &[u8]) -> Option<Self> {
        if p.len() < HEAD_LEN {
            return None;
        }
        let elf_len = u32::from_le_bytes([p[8], p[9], p[10], p[11]]) as usize;
        let cert_len = u32::from_le_bytes([p[12], p[13], p[14], p[15]]) as usize;
        let manifest_len = u32::from_le_bytes([p[16], p[17], p[18], p[19]]) as usize;
        let trailer_len = u32::from_le_bytes([p[20], p[21], p[22], p[23]]) as usize;
        let args_len = u32::from_le_bytes([p[24], p[25], p[26], p[27]]) as usize;
        let after_elf = HEAD_LEN.checked_add(elf_len)?;
        let after_cert = after_elf.checked_add(cert_len)?;
        let after_manifest = after_cert.checked_add(manifest_len)?;
        let after_trailer = after_manifest.checked_add(trailer_len)?;
        let total = after_trailer.checked_add(args_len)?;
        if p.len() != total {
            return None;
        }
        Some(Self {
            elf_len,
            cert_len,
            manifest_len,
            trailer_len,
            args_len,
            after_elf,
            after_cert,
            after_manifest,
            after_trailer,
            total,
        })
    }
}
