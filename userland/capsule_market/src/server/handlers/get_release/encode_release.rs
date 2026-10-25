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

extern crate alloc;

use alloc::vec::Vec;
use nonos_marketplace_abi::CapsuleRelease;

use crate::server::lp::{write_count, write_lp};

/// None when a field is too long for the u32 length the wire gives it.
pub(super) fn encode_release(rel: &CapsuleRelease) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    write_lp(&mut out, rel.release_id.as_bytes())?;
    out.extend_from_slice(&rel.manifest_hash);
    out.extend_from_slice(&rel.package_hash);
    write_lp(&mut out, rel.package_url.as_bytes())?;
    write_lp(&mut out, &rel.publisher_signature)?;
    write_count(&mut out, rel.supported_arches.len())?;
    for arch in &rel.supported_arches {
        write_lp(&mut out, arch.as_bytes())?;
    }
    out.extend_from_slice(&rel.kernel_abi_min.to_le_bytes());
    write_count(&mut out, rel.required_capabilities.len())?;
    for cap in &rel.required_capabilities {
        write_lp(&mut out, cap.as_bytes())?;
    }
    out.push(rel.validation.status as u8);
    write_lp(&mut out, rel.validation.note.as_bytes())?;
    write_lp(&mut out, rel.validation.validator_id.as_bytes())?;
    out.extend_from_slice(&rel.validation.validated_at_ms.to_le_bytes());
    // The escrow fields (B8, v1.4.36): zero for a release not escrowed.
    let ext = rel.ext.as_ref();
    out.extend_from_slice(&ext.map_or([0; 32], |x| x.chunk_root));
    out.extend_from_slice(&ext.map_or(0, |x| x.content_len).to_le_bytes());
    Some(out)
}
