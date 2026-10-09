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

//! Which functions are Intel VMDs, and which of them may start their child
//! buses above 0.

/// Intel VMD endpoints: Skylake/Cascade Lake server, Ice Lake server, Tiger
/// Lake, Alder Lake, Raptor Lake, Rocket Lake, Meteor Lake, Arrow Lake, Lunar
/// Lake, and the client parts Linux's vmd_ids table names after them (b07f,
/// d70b, d73b). These are class 01 subclass 04 functions that are not
/// themselves a disk controller. 28c1 is left out: Linux reads its bus range
/// from BIOS data in MEMBAR2 (VMD_FEAT_USE_BIOS_INFO), which this does not.
pub const VMD_DEVICE_IDS: [u16; 13] = [
    0x201d, 0x28c0, 0x467f, 0x4c3d, 0x7d0b, 0x9a0b, 0xa77f, 0xad0b, 0xb06f, 0xb60b, 0xb07f, 0xd70b,
    0xd73b,
];

/// VMDs whose child buses may start above 0, as VMCAP/VMCONFIG say: 28c0 and
/// every client part (VMD_FEAT_HAS_BUS_RESTRICTIONS in Linux's vmd_ids). The
/// Skylake server part always starts at bus 0 and has no such registers.
pub(super) const BUS_RESTRICTED: [u16; 12] = [
    0x28c0, 0x467f, 0x4c3d, 0x7d0b, 0x9a0b, 0xa77f, 0xad0b, 0xb06f, 0xb60b, 0xb07f, 0xd70b, 0xd73b,
];

pub const INTEL: u16 = 0x8086;

pub fn is_intel_vmd(vendor: u16, device: u16) -> bool {
    vendor == INTEL && VMD_DEVICE_IDS.contains(&device)
}
