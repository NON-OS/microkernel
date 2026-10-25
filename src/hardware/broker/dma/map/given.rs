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

//! The address a grant's device is given: an IOVA in its capsule's domain,
//! or under the development opt-in alone its physical address
//! (confine::map), then moved into the bus window the description names.

use super::super::types::{DmaMapError, DmaMapRequest};
use super::bus_window;
use super::placed::Placed;
use crate::hardware::broker::confine;
use crate::memory::iommu::IommuProtection;

/// The device address, and whether it is an IOVA; or the step that refused
/// and why, with the placement already taken back.
/// `access`: read-only or write-only for the device, by the map's direction.
pub(super) fn device_address(
    pid: u32,
    req: &DmaMapRequest,
    placed: &Placed,
    access: IommuProtection,
) -> Result<(u64, bool), (&'static str, DmaMapError)> {
    let phys = placed.phys_start;
    let (given, confined) = match confine::map(pid, req.device_id, phys, req.length, access) {
        Ok(got) => got,
        Err(no) => return Err(("confine", placed.refused(no))),
    };
    match bus_window::device_address(req.device_id, phys, req.length, given, confined) {
        Some(device_addr) => Ok((device_addr, confined)),
        None => {
            placed.undo_mapped(pid, given, confined);
            Err(("window", DmaMapError::Above4G))
        }
    }
}
