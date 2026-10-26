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

//! The record of a loaded image the rest of the loader works from.

extern crate alloc;

use alloc::format;

use crate::crypto::sig::CapsuleMetadata;
use crate::loader::image::{KernelImage, KernelSegmentLayout, MAX_KERNEL_SEGMENTS};
use crate::loader::types::memory;
use crate::log::logger::log_info;

use super::placement::Placed;

pub fn image_record(
    placed: &Placed,
    size: usize,
    segments: ([KernelSegmentLayout; MAX_KERNEL_SEGMENTS], usize),
    payload_len: usize,
) -> KernelImage {
    let image = KernelImage {
        address: placed.phys_base as usize,
        size,
        entry_point: placed.entry,
        virt_base: placed.virt_base,
        segments: segments.0,
        segment_count: segments.1,
        metadata: CapsuleMetadata {
            offset_sig: 0,
            len_sig: 0,
            offset_payload: 0,
            len_payload: payload_len,
            signer_keyid: None,
            payload_hash: [0u8; 32],
            header_version: 1,
            header_timestamp: 0,
        },
        allocations: [(0, 0); memory::MAX_ALLOCATIONS],
        alloc_count: 0,
    };
    log_info(
        "loader",
        &format!(
            "Kernel loaded: phys=0x{:x} virt=0x{:x} size=0x{:x} entry=0x{:x}",
            image.address, image.virt_base, image.size, image.entry_point,
        ),
    );
    image
}
