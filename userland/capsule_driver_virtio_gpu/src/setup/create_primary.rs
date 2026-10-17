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
use super::primary_surface;
use super::Primary;
use crate::device::virtqueue::ControlQueue;
use crate::state::{FenceCounter, ResourceTable, Scanout, ScanoutTable};

/// The primary on scanout 0, which bring-up shows; the table says so, so
/// the watch detaches it when an owner dies (G-73).
pub fn create(
    device_id: u64,
    claim_epoch: u64,
    control_queue: &ControlQueue,
    fences: &FenceCounter,
    resources: &ResourceTable,
    scanouts: &ScanoutTable,
) -> Result<Option<Primary>, &'static str> {
    let Some(s) = scanouts.get(0).filter(|s| s.enabled) else { return Ok(None) };
    let p = primary_surface::create(device_id, claim_epoch, control_queue, fences, resources, s)?;
    if let Some(p) = p {
        scanouts.record(0, Scanout { current_resource_id: p.resource_id, ..s });
    }
    Ok(p)
}
