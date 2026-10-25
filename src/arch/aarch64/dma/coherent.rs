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

//! Whether the PCI host bridge's functions snoop the CPU's caches, as the
//! tree's `dma-coherent` on the bridge says. Absent, they are taken not to:
//! a board that does snoop then pays for maintenance it did not need, where
//! the other mistake hands a device stale memory.

use core::sync::atomic::{AtomicBool, Ordering};

static PCI_COHERENT: AtomicBool = AtomicBool::new(false);

/// Boot's parse of the host bridge, once, before any function is listed.
pub(crate) fn note_pci(coherent: bool) {
    PCI_COHERENT.store(coherent, Ordering::Release);
}

pub(crate) fn pci() -> bool {
    PCI_COHERENT.load(Ordering::Acquire)
}
