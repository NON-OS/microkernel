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

//! What a map has put in place before the device is given it, and how each
//! later failure takes it back.

use super::{alloc, install};
use crate::hardware::broker::confine::{self, MapFail};
use crate::memory::iommu::IommuProtection;

pub(super) struct Placed {
    pub(super) phys_start: u64,
    pub(super) pages: u64,
    pub(super) length: u64,
    pub(super) user_va: u64,
}

impl Placed {
    /// Unmap the user pages and give the frames back.
    pub(super) fn undo(&self) {
        install::uninstall(self.user_va, self.length);
        alloc::free(self.phys_start, self.pages);
    }

    /// Give the claimed device the frames: its address and whether its domain
    /// maps them. On refusal everything placed is taken back, except frames a
    /// device may still reach through a map no unmap took away: those stay out
    /// of the allocator for good, as release leaves them.
    pub(super) fn confine(
        &self,
        pid: u32,
        device: u64,
        access: IommuProtection,
    ) -> Option<(u64, bool)> {
        match confine::map(pid, device, self.phys_start, self.length, access) {
            Ok(placed_at) => return Some(placed_at),
            Err(MapFail::Freeable) => self.undo(),
            Err(MapFail::Pinned) => install::uninstall(self.user_va, self.length),
        }
        None
    }

    /// As `undo`, once the device's domain maps the frames: they go back only
    /// if the domain gives them up. Frames it will not give up stay leaked, as
    /// release does.
    pub(super) fn undo_mapped(&self, pid: u32, device_addr: u64, confined: bool) {
        install::uninstall(self.user_va, self.length);
        if crate::hardware::broker::confine::unmap(pid, device_addr, self.length, confined) {
            alloc::free(self.phys_start, self.pages);
        }
    }
}
