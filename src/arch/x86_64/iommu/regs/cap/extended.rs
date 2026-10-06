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

/// Snoop Control, ECAP bit 7: the unit honours the snoop bit of a second-level leaf.
pub const fn snoop_control(ecap: u64) -> bool {
    ecap & (1 << 7) != 0
}

/// Queued Invalidation, ECAP bit 1: the unit takes invalidations through a
/// descriptor queue in memory instead of its CCMD and IOTLB registers.
pub const fn queued_invalidation(ecap: u64) -> bool {
    ecap & (1 << 1) != 0
}

/// Interrupt Remapping, ECAP bit 3.
pub const fn interrupt_remapping(ecap: u64) -> bool {
    ecap & (1 << 3) != 0
}

/// Extended Interrupt Mode, ECAP bit 4: remapped entries carry a 32-bit
/// x2APIC destination instead of an 8-bit xAPIC one.
pub const fn extended_interrupt_mode(ecap: u64) -> bool {
    ecap & (1 << 4) != 0
}
