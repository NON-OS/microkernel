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

use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::PhysAddr;

/* A surface window inside a range that is being unmapped. */
pub(super) struct Window {
    pub base: u64,
    /* The surface's frame list, by page index from `base`. */
    pub frames: Vec<PhysAddr>,
    /* Owner and handle when the owner unmaps a window others still map. */
    pub orphan: Option<(u32, SurfaceHandle)>,
    /* Frames of an orphaned window whose PTE was really removed. */
    pub unmapped: Vec<PhysAddr>,
}

impl Window {
    pub(super) fn held(base: u64, frames: Vec<PhysAddr>) -> Self {
        Self { base, frames, orphan: None, unmapped: Vec::new() }
    }
}
