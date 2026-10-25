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

//! The two node types a root complex's IDs are sent to, read into the
//! records the device-tree path fills for the same hardware.

use super::error::IortError;
use super::layout::{
    ITS_COUNT, ITS_IDS, RC_MIN, RC_SEGMENT, SMMU_BASE, SMMU_COHERENT, SMMU_DEVICE_ID_INDEX,
    SMMU_EVENT_GSIV, SMMU_FLAGS, SMMU_GERR_GSIV, SMMU_INDEX_VALID, SMMU_MIN, SMMU_WINDOW,
};
use super::node::Node;
use super::read::{index, u32_at, u64_at};
use super::route::handle;
use crate::arch::fdt::find::smmu::SmmuInfo;

/// An SMMUv3 node. On Arm a GSIV is the interrupt's INTID, and zero is "not
/// wired", as the tree's record has it.
pub(super) fn smmu(node: &Node) -> Result<SmmuInfo, IortError> {
    if node.bytes.len() < SMMU_MIN {
        return Err(IortError::BadNode);
    }
    let flags = u32_at(node.bytes, SMMU_FLAGS)?;
    Ok(SmmuInfo {
        base: u64_at(node.bytes, SMMU_BASE)?,
        size: SMMU_WINDOW,
        phandle: handle(node)?,
        coherent: flags & SMMU_COHERENT != 0,
        eventq_intid: u32_at(node.bytes, SMMU_EVENT_GSIV)?,
        gerror_intid: u32_at(node.bytes, SMMU_GERR_GSIV)?,
    })
}

/// The mapping that carries the SMMU's own MSIs, where the node names one:
/// it routes the SMMU's interrupts, not the devices behind it.
pub(super) fn own_msi_mapping(node: &Node) -> Result<Option<usize>, IortError> {
    if u32_at(node.bytes, SMMU_FLAGS)? & SMMU_INDEX_VALID == 0 {
        return Ok(None);
    }
    index(node.bytes, SMMU_DEVICE_ID_INDEX).map(Some)
}

/// An ITS group's first ITS. The group's ITSs share one device-ID space, so
/// any of them translates its devices' writes; the broker binds through one.
pub(super) fn first_its(node: &Node) -> Result<u32, IortError> {
    if u32_at(node.bytes, ITS_COUNT)? == 0 {
        return Err(IortError::BadNode);
    }
    u32_at(node.bytes, ITS_IDS)
}

/// A root complex's PCI segment.
pub(super) fn segment(node: &Node) -> Result<u32, IortError> {
    if node.bytes.len() < RC_MIN {
        return Err(IortError::BadNode);
    }
    u32_at(node.bytes, RC_SEGMENT)
}
