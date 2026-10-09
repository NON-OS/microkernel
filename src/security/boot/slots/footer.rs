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

//! The signed kernel file's footer, read as the loader reads it, for the two
//! regions this module needs: the kernel the loader hashed, and its proof.
//! Every rule the loader holds the footer to is held here too, so the kernel
//! never takes a region out of a file the loader would have refused.

const FOOTER_MAGIC: &[u8; 8] = b"NONOSIMG";
const FOOTER_SIZE: usize = 64;
const FOOTER_VERSION: u16 = 1;
const HASH_BLAKE3: u8 = 1;
const HAS_ZK_PROOF: u16 = 1;

/// The kernel image and its proof footer, both inside the file.
pub(super) struct Regions<'a> {
    pub kernel: &'a [u8],
    pub proof: &'a [u8],
}

/// `None` for any footer the loader's `parse_image_footer` refuses, and for a
/// file that carries no proof.
pub(super) fn regions(file: &[u8]) -> Option<Regions<'_>> {
    let start = file.len().checked_sub(FOOTER_SIZE)?;
    let f = file.get(start..)?;
    let u16_at = |i: usize| u16::from_le_bytes([f[i], f[i + 1]]);
    let u32_at = |i: usize| u64::from(u32::from_le_bytes([f[i], f[i + 1], f[i + 2], f[i + 3]]));
    let total = u64::from_le_bytes([f[16], f[17], f[18], f[19], f[20], f[21], f[22], f[23]]);
    if f.get(..8)? != FOOTER_MAGIC || u16_at(8) != FOOTER_VERSION || total != file.len() as u64 {
        return None;
    }
    if f[12] != HASH_BLAKE3 || !matches!(f[13], 1 | 2) {
        return None;
    }
    let kernel = (u32_at(24), u32_at(24) + u32_at(28));
    let signature = (u32_at(32), u32_at(32) + u32_at(36));
    let (proof_size, flags) = (u32_at(44), u16_at(10));
    let proof = (u32_at(40), u32_at(40) + proof_size);
    if overlap(kernel, signature)
        || (proof_size > 0 && (overlap(kernel, proof) || overlap(signature, proof)))
    {
        return None;
    }
    let end = start as u64;
    if kernel.1 > end || signature.1 > end || proof.1 > end {
        return None;
    }
    if proof_size == 0 || flags & HAS_ZK_PROOF == 0 {
        return None;
    }
    Some(Regions { kernel: file.get(span(kernel)?)?, proof: file.get(span(proof)?)? })
}

fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn span((from, to): (u64, u64)) -> Option<core::ops::Range<usize>> {
    Some(usize::try_from(from).ok()?..usize::try_from(to).ok()?)
}
