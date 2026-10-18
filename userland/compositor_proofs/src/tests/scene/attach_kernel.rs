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

//! The kernel for the attach proofs: the set of surfaces it still knows, as
//! the surface registry keeps it, and every map and release asked of it.

use crate::attach::SurfaceKernel;
use crate::sw_blitter::Surface;
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct FakeKernel {
    pub(crate) live: HashSet<u64>,
    /// Handles whose release the kernel refuses (the compositor holds none).
    pub(crate) refuse: HashSet<u64>,
    pub(crate) attaches: usize,
    pub(crate) released: Vec<u64>,
    /// The views the compositor holds. An attach the kernel already holds
    /// answers from it; a new one maps the surface again (`maps`). A view
    /// of a surface whose owner exited keeps its frames (an orphan) until
    /// it is released.
    pub(crate) mapped: HashSet<u64>,
    pub(crate) maps: usize,
}

impl SurfaceKernel for FakeKernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        self.attaches += 1;
        if !self.live.contains(&handle) {
            return None;
        }
        if self.mapped.insert(handle) {
            self.maps += 1;
        }
        self.live.contains(&handle).then_some(Surface {
            base_va: 0x1000 * handle,
            stride: 4,
            width: 1,
            height: 1,
            byte_len: 4,
        })
    }

    fn release(&mut self, handle: u64) -> bool {
        if self.refuse.contains(&handle) {
            return false;
        }
        self.mapped.remove(&handle);
        self.released.push(handle);
        true
    }
}
