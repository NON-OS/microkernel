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

use alloc::vec::Vec;

use super::Hmb;
use crate::admin::hmb::{descriptor, HmbPlan, DESCRIPTOR_BYTES, MAX_DESCRIPTORS, PAGE};
use crate::dma::DmaRegion;

// Every descriptor a plan can ask for fits the one list page.
const _: () = assert!(MAX_DESCRIPTORS as u64 * DESCRIPTOR_BYTES as u64 <= PAGE);

/// Map as many of the plan's pieces as the broker gives, and the one page
/// that lists them. `None` when not even the list could be mapped; the
/// caller checks the pieces against the controller's minimum.
pub(super) fn map(device_id: u64, epoch: u64, p: HmbPlan) -> Option<Hmb> {
    let mut pieces = Vec::new();
    for _ in 0..p.pieces {
        match DmaRegion::map(device_id, epoch, p.piece_pages as u64 * PAGE) {
            Ok(r) => pieces.push(r),
            Err(_) => break,
        }
    }
    let list = DmaRegion::map(device_id, epoch, PAGE).ok()?;
    for (i, piece) in pieces.iter().enumerate() {
        let d = descriptor(piece.device_addr(), p.piece_pages);
        let at = list.user_va() + (i * DESCRIPTOR_BYTES) as u64;
        // SAFETY: the plan holds at most MAX_DESCRIPTORS pieces, whose
        // 16-byte entries fill the one mapped page; no device has its
        // address yet.
        unsafe { core::ptr::copy_nonoverlapping(d.as_ptr(), at as *mut u8, DESCRIPTOR_BYTES) };
    }
    // The list is complete in memory before the command that names it is
    // submitted (the submission's own fence orders the doorbell after it).
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    Some(Hmb { list, pieces })
}
