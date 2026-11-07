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

//! A surface's frames for a device import (hardware/broker/dma/import), read
//! under the gate, so no release of the importer's attach lands between the
//! check and the import's record: `drop_attach` takes the import out under
//! the same gate before it lets the attach go.

use crate::kernel_core::surface_registry::attach_map;
use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::PhysAddr;
use crate::smp::lock_responsive;

use super::frames_of::frames_of;
use super::gate::GATE;

/// What `holder` holds of `handle`, for `with_attached`'s caller to judge:
/// whether it is attached, and the surface's frames by page (none when the
/// handle names no surface).
pub fn with_attached<R>(
    holder: u32,
    handle: SurfaceHandle,
    f: impl FnOnce(bool, &[PhysAddr]) -> R,
) -> R {
    let _gate = lock_responsive(&GATE);
    let attached = attach_map::lookup(holder, handle).is_some();
    let frames = frames_of(handle).map(|(_, frames)| frames).unwrap_or_default();
    f(attached, &frames)
}
