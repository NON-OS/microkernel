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

//! How many bytes the image needs from its lowest address: its segments'
//! span, or more when the image says so in a header at its entry. An arm64
//! kernel's stack lies past its last segment, so only its header knows.

use super::image_rules::footprint_rule;
use crate::loader::core::types::ValidationResult;
use crate::loader::errors::{LoaderError, LoaderResult};

/// The bytes at the entry point, from the segment that holds it, at most
/// `want` of them. None when no loaded segment's file bytes hold the entry.
fn head<'p>(payload: &'p [u8], v: &ValidationResult, entry: u64, want: usize) -> Option<&'p [u8]> {
    let seg = v.loads[..v.load_count]
        .iter()
        .find(|s| entry >= s.target && entry - s.target < s.p_filesz as u64)?;
    let at = seg.p_offset.checked_add((entry - seg.target) as usize)?;
    let left = seg.p_filesz - (entry - seg.target) as usize;
    payload.get(at..at.checked_add(left.min(want))?)
}

/// The segments' span, raised to what the header at the entry claims; an
/// error where the platform needs that header and the image has none.
pub fn footprint(payload: &[u8], v: &ValidationResult, span: usize) -> LoaderResult<usize> {
    let entry = v.elf.header.e_entry;
    let claimed = head(payload, v, entry, 64).and_then(crate::arch::cpu::image_size);
    let required = crate::arch::cpu::IMAGE_ALIGN.is_some();
    footprint_rule(claimed, span, required)
        .ok_or(LoaderError::UnsupportedElf("no arm64 Image header at the kernel's entry"))
}
