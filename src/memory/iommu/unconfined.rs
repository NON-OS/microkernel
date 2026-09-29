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

//! DMA that reaches a device with no IOMMU domain confining it.
//!
//! Today that is every broker grant on every machine: `MkDmaMap` hands the
//! capsule the host-physical address of its buffer and no domain takes part,
//! whichever vendor was selected. VT-d being in service does not change that,
//! because bring-up's identity domain maps all of RAM for every enumerated
//! device. So `enforcing=1` on its own would tell a reader that DMA on the
//! machine is confined when the mappings drivers actually make go around the
//! unit. The posture line carries this count beside it, and the attestation
//! document binds the pair into what the TPM signs.
//!
//! The count is of mappings still in place: a grant adds to it and its
//! release takes it back off.

use core::sync::atomic::{AtomicU32, Ordering};

static IN_PLACE: AtomicU32 = AtomicU32::new(0);

/// Mappings a device can reach with no IOMMU domain confining them, now.
pub fn unconfined_grants() -> u32 {
    IN_PLACE.load(Ordering::Acquire)
}

/// `regions` more mappings were handed to a device with no domain confining them.
pub fn note_unconfined(regions: u32) {
    IN_PLACE.fetch_add(regions, Ordering::AcqRel);
    super::posture::report_again();
}

/// `regions` unconfined mappings were taken back from their device.
pub fn note_unconfined_released(regions: u32) {
    let _ = IN_PLACE
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| Some(n.saturating_sub(regions)));
    super::posture::report_again();
}
