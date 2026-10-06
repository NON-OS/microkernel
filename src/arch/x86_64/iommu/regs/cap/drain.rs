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

/// CAP.DRD, bit 55: the unit can drain DMA reads before an IOTLB invalidation
/// completes, so a read already in flight cannot finish through a translation
/// just withdrawn. Linux sets the matching descriptor bit whenever this is set
/// (intel/dmar.c, qi_flush_iotlb).
pub const fn read_drain(cap: u64) -> bool {
    cap & (1 << 55) != 0
}

/// CAP.DWD, bit 54: as `read_drain`, for writes. A write still posted when its
/// page is unmapped would land in the frame's next owner.
pub const fn write_drain(cap: u64) -> bool {
    cap & (1 << 54) != 0
}
