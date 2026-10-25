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

//! Whether a DRHD the kernel read is kept, pure (K11). A unit left off
//! translates nothing its devices send, so while any is, the IOMMU claims
//! nothing (backend/capabilities.rs). kernel_proofs::dmar_scope.

/// Segment 0 units the kernel programs at most.
pub(crate) const MAX_REMAP_UNITS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unit {
    Kept,
    /// On a PCI segment the kernel does not enumerate, or one past the
    /// units it can hold.
    LeftOff,
}

/// The unit on `segment`, after `kept` segment 0 units.
pub(crate) const fn keep(segment: u16, kept: usize) -> Unit {
    if segment != 0 || kept >= MAX_REMAP_UNITS {
        Unit::LeftOff
    } else {
        Unit::Kept
    }
}
