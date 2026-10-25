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

use crate::hardware::broker::dma::flags::{
    DMA_MAP_COHERENT, DMA_MAP_DMA32, DMA_MAP_FROM_DEVICE, DMA_MAP_HIGH, DMA_MAP_TO_DEVICE,
    DMA_MAP_WC,
};
use crate::hardware::broker::dma::limits::dma_page_limit_for_class;
use crate::hardware::broker::dma::types::{DmaMapError, DmaMapRequest};
use crate::hardware::broker::dma::{acpi_stop::may_map, direction::conflicting};
use crate::hardware::broker::{claim, table};

pub(super) const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;
const FLAGS_KNOWN: u32 = DMA_MAP_HIGH
    | DMA_MAP_DMA32
    | DMA_MAP_COHERENT
    | DMA_MAP_WC
    | DMA_MAP_TO_DEVICE
    | DMA_MAP_FROM_DEVICE;

// Returns the claim, which the caller holds until its grant is recorded
// (L66). Nothing else is changed here.
pub(super) fn validate(req: &DmaMapRequest, pid: u32) -> Result<claim::InUse, DmaMapError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(DmaMapError::UnsupportedFlags);
    }
    // High memory and 32-bit memory are opposite asks.
    if req.flags & DMA_MAP_HIGH != 0 && req.flags & DMA_MAP_DMA32 != 0 {
        return Err(DmaMapError::UnsupportedFlags);
    }
    // One memory type per grant.
    if req.flags & DMA_MAP_COHERENT != 0 && req.flags & DMA_MAP_WC != 0 {
        return Err(DmaMapError::UnsupportedFlags);
    }
    if conflicting(req.flags) {
        return Err(DmaMapError::UnsupportedFlags);
    }
    if req.length == 0 || req.length & PAGE_MASK != 0 {
        return Err(DmaMapError::BadLength);
    }
    let claim = claim::lookup(req.device_id).ok_or(DmaMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(DmaMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(DmaMapError::StaleEpoch);
    }
    let (bus, class) = table::kind_of(req.device_id).ok_or(DmaMapError::UnknownDevice)?;
    // An ACPI kind with no stop gets no frames: its ceiling is nought (F-128).
    if !may_map(bus, class) {
        return Err(DmaMapError::BadLengthForClass);
    }
    let pages = req.length / PAGE_SIZE;
    // None against the boot framebuffer: a takeover of its scanout reclaims
    // its MMIO from outside, and a DMA grant could not be (L79).
    let fb = crate::hardware::broker::bootfb::is_boot_framebuffer(req.device_id);
    if fb || pages > dma_page_limit_for_class(class) {
        return Err(DmaMapError::BadLengthForClass);
    }
    Ok(claim)
}
