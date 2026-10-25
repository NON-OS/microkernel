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

//! What About's DMA row says, from the three fields the attestation document
//! carries and the TPM quote binds: the vendor, whether the unit enforces, and
//! how many mappings go around it. The owner's words, exactly, whenever device
//! DMA is not held: with no IOMMU in service, under the no-IOMMU posture, or
//! with a unit in service and a grant beside it.

pub const CONFINED: &[u8] = b"device DMA held by the IOMMU";
pub const UNCONFINED: &[u8] = b"device DMA not confined by hardware";

/// Held only with a unit enforcing and nothing going around it.
pub const fn confined(enforcing: bool, unconfined_grants: u32) -> bool {
    enforcing && unconfined_grants == 0
}

pub const fn words(enforcing: bool, unconfined_grants: u32) -> &'static [u8] {
    if confined(enforcing, unconfined_grants) {
        CONFINED
    } else {
        UNCONFINED
    }
}
