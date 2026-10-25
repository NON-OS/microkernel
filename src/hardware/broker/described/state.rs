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

extern crate alloc;

use alloc::vec::Vec;
use spin::RwLock;

use super::interface::Interface;
use super::iommus::DescribedIommus;
use super::window::BusWindow;

pub(super) struct Entry {
    pub device_id: u64,
    pub interface: Interface,
    /// The first register window, mapped for the kernel: the stop needs it.
    pub regs: Option<u64>,
    pub dma: Option<BusWindow>,
    pub iommus: DescribedIommus,
}

pub(super) static DESCRIBED: RwLock<Vec<Entry>> = RwLock::new(Vec::new());
